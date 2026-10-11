use super::*;
use arrow_array::{
    ArrayRef, FixedSizeListArray, Float32Array, RecordBatch, StringArray, types::Float32Type,
};
use rig_core::{
    embeddings::{Embedding, EmbeddingError},
    wasm_compat::WasmCompatSend,
};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

struct OriginEmbedding;
impl EmbeddingModel for OriginEmbedding {
    const MAX_DOCUMENTS: usize = 8;
    type Client = ();
    fn make(_: &(), _: impl Into<String>, _: Option<usize>) -> Self {
        Self
    }
    fn ndims(&self) -> usize {
        2
    }
    async fn embed_texts(
        &self,
        texts: impl IntoIterator<Item = String> + WasmCompatSend,
    ) -> Result<Vec<Embedding>, EmbeddingError> {
        Ok(texts
            .into_iter()
            .map(|document| Embedding {
                document,
                vec: vec![0.0, 0.0],
            })
            .collect())
    }
}

// Every case owns an isolated database, including cleanup after assertion failure.
struct TestDirectory(std::path::PathBuf);
impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test]
async fn id_search_preserves_native_nonzero_distances() {
    let directory = TestDirectory(std::env::temp_dir().join(format!(
        "rig-lancedb-distance-{}-{}", std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    )));
    let db = lancedb::connect(directory.0.to_str().unwrap())
        .execute()
        .await
        .unwrap();
    let vectors = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
        [
            Some(vec![Some(1.0), Some(0.0)]),
            Some(vec![Some(2.0), Some(0.0)]),
        ],
        2,
    );
    // A user column named distance must never replace the query's _distance.
    let batch = RecordBatch::try_from_iter(vec![
        (
            "id",
            Arc::new(StringArray::from(vec!["near", "far"])) as ArrayRef,
        ),
        ("embedding", Arc::new(vectors) as ArrayRef),
        (
            "distance",
            Arc::new(Float32Array::from(vec![99.0, 99.0])) as ArrayRef,
        ),
    ])
    .unwrap();
    let table = db
        .create_table("documents", vec![batch])
        .execute()
        .await
        .unwrap();
    let index = LanceDbVectorIndex::new(
        table,
        OriginEmbedding,
        "id",
        SearchParams {
            distance_type: Some(DistanceType::L2),
            search_type: Some(SearchType::Flat),
            ..SearchParams::default()
        },
    )
    .await
    .unwrap();
    let request = || {
        VectorSearchRequest::builder()
            .query("origin")
            .samples(2)
            .build()
    };
    let full = index.top_n::<Value>(request()).await.unwrap();
    assert_eq!(
        full.iter()
            .map(|(distance, id, _)| (*distance, id.as_str()))
            .collect::<Vec<_>>(),
        vec![(1.0, "near"), (4.0, "far")]
    );
    let ids = index.top_n_ids(request()).await.unwrap();
    assert_eq!(ids, vec![(1.0, "near".to_owned()), (4.0, "far".to_owned())]);
}
