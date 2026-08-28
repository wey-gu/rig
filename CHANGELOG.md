# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.40.0](https://github.com/wey-gu/rig/compare/v0.39.0...v0.40.0) - 2026-08-28

### Added

- *(openai)* preserve responses prompt cache parameters ([#1830](https://github.com/wey-gu/rig/pull/1830)) (by @Kade-Powell)
- *(streaming)* [**breaking**] surface unmodeled provider output items through the stream ([#1951](https://github.com/wey-gu/rig/pull/1951)) (by @gold-silver-copper)
- *(rig-core)* [**breaking**] integrate hooks into AgentRun via a composable AgentRunner ([#1945](https://github.com/wey-gu/rig/pull/1945)) (by @gold-silver-copper)
- *(message)* add video helper constructors + OpenRouter audio/video conversion tests ([#1942](https://github.com/wey-gu/rig/pull/1942)) (by @gold-silver-copper)
- *(agent)* add OutputMode to compose structured output with tools ([#1928](https://github.com/wey-gu/rig/pull/1928)) ([#1929](https://github.com/wey-gu/rig/pull/1929)) (by @gold-silver-copper)
- *(providers)* add VoyageAI rerank support ([#1917](https://github.com/wey-gu/rig/pull/1917)) (by @sergiomeneses)
- *(agent)* [**breaking**] sans-IO AgentRun state machine; both agent loops become thin drivers ([#1899](https://github.com/wey-gu/rig/pull/1899)) (by @gold-silver-copper)
- *(rig-derive)* replace hand-rolled schema with schemars in #[rig_tool] ([#1576](https://github.com/wey-gu/rig/pull/1576)) (by @tomasz-feliksik)
- *(gemini)* expose streaming response metadata ([#1790](https://github.com/wey-gu/rig/pull/1790)) (by @mateobelanger)
- *(anthropic)* support document citations ([#1778](https://github.com/wey-gu/rig/pull/1778))
- *(openrouter)* add transcription (STT) and audio generation (TTS) support ([#1757](https://github.com/wey-gu/rig/pull/1757)) (by @fversaci)
- *(rig-bedrock)* add structured output support via Converse API ([#1667](https://github.com/wey-gu/rig/pull/1667)) (by @jdwil)
- *(memory)* Rig-managed conversation memory + rig-memory companion crate ([#1702](https://github.com/wey-gu/rig/pull/1702)) (by @ForeverAngry)
- add copilot model listing ([#1700](https://github.com/wey-gu/rig/pull/1700)) (by @BigtoC) - #1700
- *(core)* add Xiaomi MiMo ([#1685](https://github.com/wey-gu/rig/pull/1685)) (by @BigtoC)
- rustls by default for everything ([#1682](https://github.com/wey-gu/rig/pull/1682)) (by @gold-silver-copper) - #1682
- gpt image 2, gpt 5.5, opus 4.7  ([#1679](https://github.com/wey-gu/rig/pull/1679)) (by @gold-silver-copper) - #1679
- *(core)* add DeepSeek model listing api ([#1672](https://github.com/wey-gu/rig/pull/1672)) (by @BigtoC)
- *(deepseek)* deprecate old model names ([#1664](https://github.com/wey-gu/rig/pull/1664)) (by @fu050409)
- *(ollama)* allow setting base_url and api_key programmatically ([#1511](https://github.com/wey-gu/rig/pull/1511)) (by @majiayu000)
- *(rig-core)* add ChatGPT Subscription, GitHub Copilot, and compatibility providers ([#1615](https://github.com/wey-gu/rig/pull/1615)) (by @wey-gu)
- *(providers)* add GitHub Copilot provider with relaxed response parsing ([#1451](https://github.com/wey-gu/rig/pull/1451)) (by @DAMEK86)
- *(rig-core)* Add model listing capability to OpenRouter client ([#1627](https://github.com/wey-gu/rig/pull/1627)) (by @nate-trojian)
- *(rig-derive)* support custom tool names ([#1619](https://github.com/wey-gu/rig/pull/1619)) ([#1620](https://github.com/wey-gu/rig/pull/1620)) (by @qaqland)
- *(rig-bedrock)* add OpenTelemetry tracing to completion model ([#1567](https://github.com/wey-gu/rig/pull/1567)) (by @sachin-punyani)
- *(rig-1197)* handle llama.cpp tool call ([#1408](https://github.com/wey-gu/rig/pull/1408)) ([#1409](https://github.com/wey-gu/rig/pull/1409)) (by @inqode-lars)
- *(rig-core)* respect custom Authorization headers set via http_headers() ([#1553](https://github.com/wey-gu/rig/pull/1553)) (by @shlaikov)
- make history generic and immutable ([#1563](https://github.com/wey-gu/rig/pull/1563)) (by @FayCarsons) - #1563
- add grok xAI TTS ([#1530](https://github.com/wey-gu/rig/pull/1530)) (by @lmariscal) - #1530
- *(rig-sqlite)* impl InsertDocuments trait for sqlite ([#1478](https://github.com/wey-gu/rig/pull/1478)) (by @howenyap)
- *(rig-core)* add stateful WebSocket session for OpenAI Responses API ([#1500](https://github.com/wey-gu/rig/pull/1500)) (by @illegalcall)
- *(gemini)* add Gemini 3 model constants and thinking_level support ([#1520](https://github.com/wey-gu/rig/pull/1520)) (by @EinarasGar)
- add llamafile provider ([#1519](https://github.com/wey-gu/rig/pull/1519)) (by @Sarah-2003) - #1519
- add grok imagine as image generation ([#1516](https://github.com/wey-gu/rig/pull/1516)) (by @lmariscal) - #1516
- *(rmcp)* `McpClientHandler` ([#1525](https://github.com/wey-gu/rig/pull/1525)) (by @joshua-mo-143)
- *(telemetry)* emit gen_ai.usage.cached_tokens across all providers ([#1497](https://github.com/wey-gu/rig/pull/1497)) (by @alwayys-afk)
- add provider-native hosted tool support ([#1430](https://github.com/wey-gu/rig/pull/1430)) (by @bnomei) - #1430
- *(moonshot)* add Kimi K2 and K2.5 model constants ([#1457](https://github.com/wey-gu/rig/pull/1457)) (by @howardpen9)
- *(gemini)* add RAG extractor example and dynamic_context support ([#1456](https://github.com/wey-gu/rig/pull/1456)) (by @atellou)
- *(gemini)* Add support for RAG documents in dynamic context ([#1205](https://github.com/wey-gu/rig/pull/1205)) (by @snaumov)
- *(rig-core)* return conversation messages from non-streaming agent loop ([#1450](https://github.com/wey-gu/rig/pull/1450)) (by @illegalcall)
- *(extractor)* expose token usage via extract_with_usage methods ([#1447](https://github.com/wey-gu/rig/pull/1447)) (by @liamwh)
- add `.extended_details` to `TypedPromptRequest` via typestate ([#1446](https://github.com/wey-gu/rig/pull/1446)) (by @0xMochan) - #1446
- Add Cloudflare Vectorize integration to rig  ([#1417](https://github.com/wey-gu/rig/pull/1417)) (by @AjithPanneerselvam) - #1417
- *(mistral)* implements audio transcription api ([#1424](https://github.com/wey-gu/rig/pull/1424)) (by @renanvieira)
- Reify SSE state machine ([#1428](https://github.com/wey-gu/rig/pull/1428)) (by @FayCarsons) - #1428
- feat(openrouter) Add support for openrouter embeddings ([#1418](https://github.com/wey-gu/rig/pull/1418)) ([#1419](https://github.com/wey-gu/rig/pull/1419)) (by @Lochlanna) - #1419
- *(azure-openai)* Add structured outputs support ([#1407](https://github.com/wey-gu/rig/pull/1407)) (by @austinsimpsond41)
- *(openrouter)* support audio and video ([#1413](https://github.com/wey-gu/rig/pull/1413)) (by @micllam)
- *(rig-1192)* support pdf, image (openrouter) ([#1404](https://github.com/wey-gu/rig/pull/1404)) (by @joshua-mo-143)
- cross-provider reasoning trace roundtrip ([#1396](https://github.com/wey-gu/rig/pull/1396)) (by @darinkishore) - #1396
- *(openrouter)* Add provider selection and prioritization support ([#1373](https://github.com/wey-gu/rig/pull/1373)) (by @milutinke)
- *(rig-1189)* structured outputs ([#1382](https://github.com/wey-gu/rig/pull/1382)) (by @joshua-mo-143)
- *(rig-core)* add optional model override to CompletionRequest ([#1374](https://github.com/wey-gu/rig/pull/1374)) (by @illegalcall)
- *(rig-1180)* support text docs (anthropic) ([#1377](https://github.com/wey-gu/rig/pull/1377)) (by @joshua-mo-143)
- Add model listing capability ([#1243](https://github.com/wey-gu/rig/pull/1243)) (by @eibrahim95) - #1243
- *(rig-1168)* add default prompt hook to agent (breaking) ([#1356](https://github.com/wey-gu/rig/pull/1356)) (by @joshua-mo-143)
- [**breaking**] upgrade reqwest to 0.13 with rustls as default TLS backend ([#1218](https://github.com/wey-gu/rig/pull/1218)) (by @FrancescoXX) - #1218
- *(rig-1182)* single-text serialization to single string (openai) ([#1367](https://github.com/wey-gu/rig/pull/1367)) (by @joshua-mo-143)
- add reqwest middleware example ([#1359](https://github.com/wey-gu/rig/pull/1359)) (by @FayCarsons) - #1359
- *(providers)* expose cached_input_tokens in Usage struct ([#1299](https://github.com/wey-gu/rig/pull/1299)) (by @legietha)
- *(tools)* adds internal tool call id (breaking) ([#1311](https://github.com/wey-gu/rig/pull/1311)) (by @mylesberueda)
- *(rig-1160)* encode control flow directly in type signature for hooks (breaking) ([#1305](https://github.com/wey-gu/rig/pull/1305)) (by @joshua-mo-143)
- *(rig-1126)* tool image result support for gemini ([#1329](https://github.com/wey-gu/rig/pull/1329)) (by @joshua-mo-143)
- support xhigh reasoning effort ([#1319](https://github.com/wey-gu/rig/pull/1319)) (by @acgxv) - #1319
- *(1159)* impl VectorStoreIndexDyn for scyllaDB integration ([#1302](https://github.com/wey-gu/rig/pull/1302)) (by @FayCarsons)
- *(agent)* allow on_tool_call hook to reject tool execution ([#1284](https://github.com/wey-gu/rig/pull/1284)) (by @Phoenix500526)
- improve vector store documentation and filter ergonomics (breaking) ([#1258](https://github.com/wey-gu/rig/pull/1258)) (by @FayCarsons) - #1258
- *(rig-1142)* include agent names in tracing ([#1270](https://github.com/wey-gu/rig/pull/1270)) (by @joshua-mo-143)
- *(rig-1144)* deepseek reasoning content (non-streaming) ([#1269](https://github.com/wey-gu/rig/pull/1269)) (by @joshua-mo-143)
- *(rig-1147)* re-export reqwest client ([#1277](https://github.com/wey-gu/rig/pull/1277)) (by @joshua-mo-143)
- add custom vector store backend example ([#1252](https://github.com/wey-gu/rig/pull/1252)) (by @FayCarsons) - #1252
- add default max depth to agents ([#1253](https://github.com/wey-gu/rig/pull/1253)) (by @0xMochan) - #1253
- Add the `user` parameter to openai-embedding. ([#1254](https://github.com/wey-gu/rig/pull/1254)) (by @felix307253927) - #1254
- *(rig-1135)* Agentic loop early termination reason ([#1248](https://github.com/wey-gu/rig/pull/1248)) (by @joshua-mo-143)
- make integration filters available to be used as rig agent rag store ([#1249](https://github.com/wey-gu/rig/pull/1249)) (by @joshua-mo-143) - #1249
- Add the ```encoding_format``` parameter to openai-embedding. ([#1203](https://github.com/wey-gu/rig/pull/1203)) (by @felix307253927) - #1203
- *(agent)* export StreamingError to public API ([#1200](https://github.com/wey-gu/rig/pull/1200)) (by @Phoenix500526)
- *(rig-1096)* pass tool call ID to prompt hook ([#1162](https://github.com/wey-gu/rig/pull/1162)) (by @joshua-mo-143)
- *(rig-1059)* support `reqwest-middleware` client ([#1152](https://github.com/wey-gu/rig/pull/1152)) (by @joshua-mo-143)
- Introduce reasoning delta ([#1055](https://github.com/wey-gu/rig/pull/1055)) (by @Sytten) - #1055
- *(rig-1066)* concurrent execution for tools ([#1142](https://github.com/wey-gu/rig/pull/1142)) (by @joshua-mo-143)
- LSH for in memory vector store ([#922](https://github.com/wey-gu/rig/pull/922)) (by @Dev79844) - #922
- add Anthropic prompt caching support ([#1116](https://github.com/wey-gu/rig/pull/1116))
- *(rig-1076)* Providers should route all requests through `client::Client` ([#1115](https://github.com/wey-gu/rig/pull/1115)) (by @FayCarsons)
- Gemini Assistant Image Responses ([#1048](https://github.com/wey-gu/rig/pull/1048)) (by @erlendp) - #1048
- *(gemini-request)* add response_json_schema to GenerationConfig ([#1077](https://github.com/wey-gu/rig/pull/1077)) (by @Numeration)
- *(rig-985)* Consolidate provider clients ([#1050](https://github.com/wey-gu/rig/pull/1050)) (by @FayCarsons)
- rig-vertexai ([#1040](https://github.com/wey-gu/rig/pull/1040)) (by @kafonek) - #1040
- *(rig-1014)* add backend specific vector search filters ([#1032](https://github.com/wey-gu/rig/pull/1032)) (by @FayCarsons)
- *(rig-1024)* yield tool calls and results from multi-step stream prompt ([#1023](https://github.com/wey-gu/rig/pull/1023)) (by @joshua-mo-143)
- *(providers)* Emit tool call deltas ([#1020](https://github.com/wey-gu/rig/pull/1020)) (by @scrogson)
- export rig tool macro from main crate ([#1016](https://github.com/wey-gu/rig/pull/1016)) (by @joshua-mo-143) - #1016
- *(rig-1021)* allow language to be set to None for transcription ([#997](https://github.com/wey-gu/rig/pull/997)) (by @joshua-mo-143)
- *(rig-1008)* add Send + Sync to ProviderClient ([#974](https://github.com/wey-gu/rig/pull/974)) (by @joshua-mo-143)
- *(rig-976)* support filters for `VectorSearchRequest` ([#952](https://github.com/wey-gu/rig/pull/952)) (by @FayCarsons)
- *(rig-1004)* expose tool call partials ([#960](https://github.com/wey-gu/rig/pull/960)) (by @joshua-mo-143)
- convert video media mime type ([#959](https://github.com/wey-gu/rig/pull/959)) (by @joshua-mo-143) - #959
- *(rig-996)* generic streaming ([#955](https://github.com/wey-gu/rig/pull/955)) (by @joshua-mo-143)
- *(gemini)* Support streaming thinking ([#947](https://github.com/wey-gu/rig/pull/947)) (by @scrogson)
- *(bedrock)* Support streaming thinking ([#946](https://github.com/wey-gu/rig/pull/946)) (by @scrogson)
- *(ollama)* thinking ([#948](https://github.com/wey-gu/rig/pull/948)) (by @scrogson)
- *(bedrock)* Implement usage ([#934](https://github.com/wey-gu/rig/pull/934)) (by @scrogson)
- *(anthropic)* Expose the reasoning signature ([#945](https://github.com/wey-gu/rig/pull/945)) (by @scrogson)
- Implement thinking for anthropic streaming ([#928](https://github.com/wey-gu/rig/pull/928)) (by @scrogson) - #928
- *(rig-937)* evals ([#905](https://github.com/wey-gu/rig/pull/905)) (by @joshua-mo-143)
- *(rig-986)* tool servers ([#916](https://github.com/wey-gu/rig/pull/916)) (by @joshua-mo-143)
- *(rig-988)* cancel streaming prompts from prompt hook ([#918](https://github.com/wey-gu/rig/pull/918)) (by @joshua-mo-143)
- *(rig-990)* allow configuring optional lancedb features ([#923](https://github.com/wey-gu/rig/pull/923)) (by @joshua-mo-143)
- return usage when streaming completions from a dynamic client ([#903](https://github.com/wey-gu/rig/pull/903)) (by @lefebvreb) - #903
- *(rig-979)* discord bot integration ([#900](https://github.com/wey-gu/rig/pull/900)) (by @joshua-mo-143)
- *(rig-935)* support cancelling multi-turn prompt loop from hook ([#904](https://github.com/wey-gu/rig/pull/904)) (by @joshua-mo-143)
- *(rig-951)* generic HTTP client ([#875](https://github.com/wey-gu/rig/pull/875)) (by @FayCarsons)
- *(rig-841)* support HelixDB ([#805](https://github.com/wey-gu/rig/pull/805)) (by @joshua-mo-143)
- *(rig-977)* add description field to Agent, update tool impl ([#895](https://github.com/wey-gu/rig/pull/895)) (by @joshua-mo-143)
- *(rig-848)* extract JSON with chat history ([#888](https://github.com/wey-gu/rig/pull/888)) (by @joshua-mo-143)
- *(rig-955)* set up tool choice capability for Extractor ([#884](https://github.com/wey-gu/rig/pull/884)) (by @joshua-mo-143)
- *(rig-964)* add tool choice to agent ([#883](https://github.com/wey-gu/rig/pull/883)) (by @joshua-mo-143)
- *(rig-973)* DocumentSourceKind::String ([#882](https://github.com/wey-gu/rig/pull/882)) (by @joshua-mo-143)
- GenAI SemConv support (otel) ([#850](https://github.com/wey-gu/rig/pull/850)) (by @joshua-mo-143) - #850
- add streaming support to DynClientBuilder ([#824](https://github.com/wey-gu/rig/pull/824)) (by @sternelee) - #824
- *(rig-912)* rework `Chat` trait for multi-turn ([#846](https://github.com/wey-gu/rig/pull/846)) (by @FayCarsons)
- *(rig-795)* support file URLs for audio, video, documents ([#823](https://github.com/wey-gu/rig/pull/823)) (by @FayCarsons)
- *(rig-943)* support thinking for cohere ([#827](https://github.com/wey-gu/rig/pull/827)) (by @joshua-mo-143)
- Nix flake for development environment ([#831](https://github.com/wey-gu/rig/pull/831)) (by @FayCarsons) - #831
- think tool, vector store tool, better agent tool ([#424](https://github.com/wey-gu/rig/pull/424)) (by @0xMochan) - #424
- *(rig-926)* make agent multi stream prompting more granular ([#796](https://github.com/wey-gu/rig/pull/796)) (by @joshua-mo-143)
- *(rig-928)* allow openai chat completions to be used as an extractor ([#797](https://github.com/wey-gu/rig/pull/797)) (by @joshua-mo-143)
- *(rig-831)* ensure all features are added to docs.rs ([#801](https://github.com/wey-gu/rig/pull/801)) (by @joshua-mo-143)
- *(rig-931)* support file input for images on Gemini ([#790](https://github.com/wey-gu/rig/pull/790)) (by @joshua-mo-143)
- *(rig-core)* add fn cli_chatbot() back ([#769](https://github.com/wey-gu/rig/pull/769)) (by @joshua-mo-143)
- *(rig-918)* expose more token usage metadata metrics for gemini ([#768](https://github.com/wey-gu/rig/pull/768)) (by @joshua-mo-143)
- *(rig-911)* ConvertMessage trait ([#753](https://github.com/wey-gu/rig/pull/753)) (by @joshua-mo-143)
- *(rig-915)* function calling strict mode for deepseek ([#763](https://github.com/wey-gu/rig/pull/763)) (by @joshua-mo-143)
- *(openai responses)* add `minimal` variant to ReasoningEffort ([#765](https://github.com/wey-gu/rig/pull/765)) (by @joshua-mo-143)
- *(rig-904)* Rework CLI chatbot integration ([#756](https://github.com/wey-gu/rig/pull/756)) (by @FayCarsons)
- Pauseable streams ([#733](https://github.com/wey-gu/rig/pull/733)) (by @Dev79844) - #733
- *(rig-910)* function calls fail when using OpenAI Responses API with reasoning models ([#754](https://github.com/wey-gu/rig/pull/754)) (by @joshua-mo-143)
- *(rig-901)* Make multi-turn stream return a `Send + 'static` stream ([#739](https://github.com/wey-gu/rig/pull/739)) (by @joshua-mo-143)
- VerifyClient trait ([#724](https://github.com/wey-gu/rig/pull/724)) (by @Sytten) - #724
- *(rig-898)* make MultiTurnStreamItem pub ([#735](https://github.com/wey-gu/rig/pull/735)) (by @joshua-mo-143)
- *(rig-865)* multi turn streaming ([#712](https://github.com/wey-gu/rig/pull/712)) (by @joshua-mo-143)
- implement Tool for Agent ([#704](https://github.com/wey-gu/rig/pull/704)) (by @dmackdev) - #704
- Add capability to add custom logic while running prompts ([#632](https://github.com/wey-gu/rig/pull/632)) (by @morenol) - #632
- *(rig-863)* add retries to extractor tool ([#685](https://github.com/wey-gu/rig/pull/685)) (by @joshua-mo-143)
- *(gemini)* Accept plain-text tool result ([#686](https://github.com/wey-gu/rig/pull/686)) (by @quangIO)
- video input for gemini ([#690](https://github.com/wey-gu/rig/pull/690)) (by @yavens) - #690
- added get_tool_definitions ([#666](https://github.com/wey-gu/rig/pull/666)) (by @rajiknows) - #666
- *(rig-845)* cosine similarity for vector search ([#664](https://github.com/wey-gu/rig/pull/664)) (by @joshua-mo-143)
- add `delete_tool` method to `Toolset` ([#663](https://github.com/wey-gu/rig/pull/663)) (by @rajiknows) - #663
- Read the OPENAI_BASE_URL env variable when constructing an OpenAI client from_env ([#659](https://github.com/wey-gu/rig/pull/659)) (by @sredman) - #659
- add agent name ([#633](https://github.com/wey-gu/rig/pull/633)) (by @morenol) - #633
- *(rig-798)* `rig-wasm` ([#611](https://github.com/wey-gu/rig/pull/611)) (by @joshua-mo-143)
- *(rig-819)* vector store index request struct ([#623](https://github.com/wey-gu/rig/pull/623)) (by @joshua-mo-143)
- *(rig-830)* map documents to text for OpenAI Response API ([#622](https://github.com/wey-gu/rig/pull/622)) (by @joshua-mo-143)
- Add GROK_4 model constant to xAI provider ([#614](https://github.com/wey-gu/rig/pull/614)) (by @hghalebi) - #614
- *(rig-812)* yield final response with total usage metrics from streaming completion response in stream impl ([#584](https://github.com/wey-gu/rig/pull/584)) (by @joshua-mo-143)
- *(rig-799)* add support for official rust sdk for mcp ([#553](https://github.com/wey-gu/rig/pull/553)) (by @joshua-mo-143)
- *(rig-823)* impl size hint for OneOrMany types ([#606](https://github.com/wey-gu/rig/pull/606)) (by @joshua-mo-143)
- *(rig-784)* thinking/reasoning ([#557](https://github.com/wey-gu/rig/pull/557)) (by @joshua-mo-143)
- *(rig-821)* add tracing when submit tool is never called in extractor ([#603](https://github.com/wey-gu/rig/pull/603)) (by @joshua-mo-143)
- *(rig-816)* add support for AWS S3Vectors ([#597](https://github.com/wey-gu/rig/pull/597)) (by @joshua-mo-143)
- *(rig-mongodb)* implement InsertDocuments trait ([#596](https://github.com/wey-gu/rig/pull/596)) (by @ksaritek)
- make PromptResponse public ([#593](https://github.com/wey-gu/rig/pull/593)) (by @morenol) - #593
- *(rig-801)* DynClientBuilder::from_values ([#556](https://github.com/wey-gu/rig/pull/556)) (by @joshua-mo-143)
- add `.extended_details` to `PromptRequest` ([#555](https://github.com/wey-gu/rig/pull/555)) (by @0xMochan) - #555
- support inserting documents as a trait ([#563](https://github.com/wey-gu/rig/pull/563)) (by @rajiknows) - #563
- Add max_tokens method to ExtractorBuilder ([#560](https://github.com/wey-gu/rig/pull/560)) (by @snowmead) - #560
- *(rig-780)* integrate openAI responses API ([#508](https://github.com/wey-gu/rig/pull/508)) (by @joshua-mo-143)
- Stream cancellation using AbortHandle ([#525](https://github.com/wey-gu/rig/pull/525)) (by @rajiknows) - #525
- add ScyllaDB vector store implementation ([#533](https://github.com/wey-gu/rig/pull/533)) (by @ksaritek) - #533
- *(rig-779)* allow extractor to be turned into inner agent ([#502](https://github.com/wey-gu/rig/pull/502)) (by @joshua-mo-143)
- *(ollama)* add support for OLLAMA_API_BASE_URL environment var ([#541](https://github.com/wey-gu/rig/pull/541)) (by @rmsc)
- *(rig-766)* add support for Voyage AI ([#493](https://github.com/wey-gu/rig/pull/493)) (by @joshua-mo-143)
- *(rig-789)* add support for loading in pdfs/files as Vec<u8> ([#523](https://github.com/wey-gu/rig/pull/523)) (by @joshua-mo-143)
- multi turn streaming example ([#413](https://github.com/wey-gu/rig/pull/413)) (by @JazzyMcJazz) - #413
- *(rig-754)* support custom client configurations ([#511](https://github.com/wey-gu/rig/pull/511)) (by @joshua-mo-143)
- add additional Gemini completion models ([#498](https://github.com/wey-gu/rig/pull/498)) (by @davisuga) - #498
- *(rig-762)* update fastembed version ([#480](https://github.com/wey-gu/rig/pull/480)) (by @joshua-mo-143)
- *(rig-758)* the extractor can pass additional params to be passed to the model ([#473](https://github.com/wey-gu/rig/pull/473)) (by @joshua-mo-143)
- gitignore .envrc ([#474](https://github.com/wey-gu/rig/pull/474)) (by @joshua-mo-143) - #474
- *(rig-744)* Add support for Milvus vector store ([#463](https://github.com/wey-gu/rig/pull/463)) (by @joshua-mo-143)
- Improve Streaming API ([#388](https://github.com/wey-gu/rig/pull/388)) (by @yavens) - #388
- add gpt-image-1 ([#418](https://github.com/wey-gu/rig/pull/418)) (by @yavens) - #418
- multi-turn / reasoning loops + parallel tool calling ([#370](https://github.com/wey-gu/rig/pull/370)) (by @0xMochan) - #370
- support custom clients for bedrock ([#403](https://github.com/wey-gu/rig/pull/403)) (by @marcbowes) - #403
- trait for embedding images ([#396](https://github.com/wey-gu/rig/pull/396)) (by @joshua-mo-143) - #396
- Add `rig_tool` macro ([#353](https://github.com/wey-gu/rig/pull/353)) (by @cbrit) - #353
- impl From<mcp_core::types::Tool> for ToolDefinition ([#385](https://github.com/wey-gu/rig/pull/385)) (by @joshua-mo-143) - #385
- AWS Bedrock provider ([#318](https://github.com/wey-gu/rig/pull/318)) (by @mkranjac) - #318
- Add audio generation to all providers ([#359](https://github.com/wey-gu/rig/pull/359)) (by @yavens) - #359
- Add image generation to all providers that support it ([#357](https://github.com/wey-gu/rig/pull/357)) (by @yavens) - #357
- *(rig-fastembed)* support loading model files from custom path ([#337](https://github.com/wey-gu/rig/pull/337)) (by @zhongjingjogy)
- *(provider)* cohere-v2 ([#350](https://github.com/wey-gu/rig/pull/350)) (by @0xMochan)
- Add streaming to all model providers ([#347](https://github.com/wey-gu/rig/pull/347)) (by @yavens) - #347
- OpenRouter support ([#344](https://github.com/wey-gu/rig/pull/344)) (by @joshua-mo-143) - #344
- add reqwest/rustls-tls support ([#339](https://github.com/wey-gu/rig/pull/339)) (by @jerryshell) - #339
- add transcription to all providers that support it ([#336](https://github.com/wey-gu/rig/pull/336)) (by @yavens) - #336
- Azure OpenAI Token Authentication ([#329](https://github.com/wey-gu/rig/pull/329)) (by @Verticaleap) - #329
- SSE/JSONL decoders ported from Anthropic TS SDK ([#332](https://github.com/wey-gu/rig/pull/332)) (by @piotrostr) - #332
- mira integration ([#282](https://github.com/wey-gu/rig/pull/282)) (by @VaibhavSatija10) - #282
- Huggingface provider integration ([#321](https://github.com/wey-gu/rig/pull/321)) (by @yavens) - #321
- Transcription Model support ([#322](https://github.com/wey-gu/rig/pull/322)) (by @yavens) - #322
- Add EpubFileLoader for EPUB file processing ([#192](https://github.com/wey-gu/rig/pull/192)) (by @danik-tro) - #192
- add ollama client ([#285](https://github.com/wey-gu/rig/pull/285)) (by @451846939) - #285
- surrealdb integration ([#280](https://github.com/wey-gu/rig/pull/280)) (by @joshua-mo-143) - #280
- *(openai)* add updated OpenAI model constants ([#314](https://github.com/wey-gu/rig/pull/314)) (by @affaan-m)
- support together AI ([#230](https://github.com/wey-gu/rig/pull/230)) (by @threewebcode)
- upsert documents into qdrant ([#301](https://github.com/wey-gu/rig/pull/301)) (by @joshua-mo-143) - #301
- *(streaming)* add `Send` to `StreamingResult` inner Stream ([#302](https://github.com/wey-gu/rig/pull/302)) (by @piotrostr)
- groq integration ([#263](https://github.com/wey-gu/rig/pull/263)) (by @joshua-mo-143) - #263
- fastembed integration ([#268](https://github.com/wey-gu/rig/pull/268)) (by @joshua-mo-143) - #268
- *(core)* overhaul message API ([#199](https://github.com/wey-gu/rig/pull/199)) (by @0xMochan)
- Add support for Azure OpenAI ([#234](https://github.com/wey-gu/rig/pull/234)) (by @jimmydottech) - #234
- support moonshot language model ([#223](https://github.com/wey-gu/rig/pull/223)) (by @threewebcode)
- galadriel api integration (redux) ([#265](https://github.com/wey-gu/rig/pull/265)) (by @joshua-mo-143) - #265
- add Galadriel API integration ([#188](https://github.com/wey-gu/rig/pull/188)) (by @kristjanpeterson1) - #188
- support extractor for deepseek ([#255](https://github.com/wey-gu/rig/pull/255)) (by @lispking) - #255
- support tools for DeepSeek provider ([#251](https://github.com/wey-gu/rig/pull/251)) (by @carlos-verdes) - #251
- streaming API implementation for Anthropic provider ([#232](https://github.com/wey-gu/rig/pull/232)) (by @piotrostr) - #232
- *(rig-postgres)* postgres vector store integration ([#231](https://github.com/wey-gu/rig/pull/231)) (by @carlos-verdes)
- Add hyperbolic inference API integration ([#238](https://github.com/wey-gu/rig/pull/238)) (by @joshua-mo-143) - #238
- *(rig-eternalai)* add support for EternalAI onchain toolset ([#205](https://github.com/wey-gu/rig/pull/205))
- *(pipeline)* Add conditional op ([#200](https://github.com/wey-gu/rig/pull/200)) (by @AdrielC)
- Add support for DeepSeek ([#220](https://github.com/wey-gu/rig/pull/220)) (by @wale-e) - #220
- Add `from_url` method to Gemini client ([#194](https://github.com/wey-gu/rig/pull/194)) (by @cgfo) - #194
- Feature flag for CF worker compatibility ([#176](https://github.com/wey-gu/rig/pull/176)) ([#175](https://github.com/wey-gu/rig/pull/175)) (by @ronaldslc) - #175
- *(eternal-ai)* Eternal-AI provider for rig ([#171](https://github.com/wey-gu/rig/pull/171))
- Add gpt-4o-mini to openai model list ([#187](https://github.com/wey-gu/rig/pull/187)) (by @zhourunlai) - #187
- agent pipelines ([#131](https://github.com/wey-gu/rig/pull/131)) (by @cvauclair) - #131
- *(rig-anthropic)* Add default `max_tokens` for standard models ([#151](https://github.com/wey-gu/rig/pull/151)) (by @0xMochan)
- Add support for Sqlite vector store ([#122](https://github.com/wey-gu/rig/pull/122)) (by @tarrencev) - #122
- Improve `InMemoryVectorStore` API ([#130](https://github.com/wey-gu/rig/pull/130)) (by @cvauclair) - #130
- embeddings API overhaul ([#120](https://github.com/wey-gu/rig/pull/120)) (by @marieaurore123) - #120
- *(provider)* xAI (grok) integration ([#106](https://github.com/wey-gu/rig/pull/106)) (by @0xMochan)
- *(gemini)* move system prompt to correct request field (by @mateobelanger)
- *(provider-gemini)* add support for gemini specific completion parameters
- *(provider-gemini)* add agent support in client
- *(provider-gemini)* add gemini embedding support
- *(provider-gemini)* add gemini support for basic completion
- *(provider-gemini)* add gemini API client
- anthropic models (by @0xMochan) - #27
- add o1-preview and o1-mini (by @0xMochan) - #19
- Add arbitrary prompt error (by @cvauclair)
- Add model name enums for existing providers (by @cvauclair)

### Fixed

- *(rig-core)* fix epub loader tests + prevent CWD-relative fixture-path regressions ([#1940](https://github.com/wey-gu/rig/pull/1940)) (by @gold-silver-copper)
- *(ollama)* preserve assistant reasoning from non-streaming responses ([#1926](https://github.com/wey-gu/rig/pull/1926)) ([#1927](https://github.com/wey-gu/rig/pull/1927)) (by @gold-silver-copper)
- correct possessive pronoun typo in CONTRIBUTING.md ([#1865](https://github.com/wey-gu/rig/pull/1865)) (by @abhicris) - #1865
- *(tool)* [**breaking**] deterministic, duplicate-safe tool registration + cassette tests ([#1913](https://github.com/wey-gu/rig/pull/1913)) (by @gold-silver-copper)
- support Anthropic mid-conversation system role ([#1862](https://github.com/wey-gu/rig/pull/1862)) (by @fangkangmi) - #1862
- *(chatgpt)* Handle ChatGPT response.completed events without output field ([#1825](https://github.com/wey-gu/rig/pull/1825)) (by @geraschenko)
- *(rig-gemini-grpc)* populate FunctionDeclaration.parameters from ToolDefinition ([#1763](https://github.com/wey-gu/rig/pull/1763)) (by @abhicris)
- fix sqlite threshold and null tool call streaming ([#1786](https://github.com/wey-gu/rig/pull/1786)) (by @gold-silver-copper) - #1786
- *(gemini)* Token usage correctness for posthog llm analytics ([#1761](https://github.com/wey-gu/rig/pull/1761)) (by @mateobelanger)
- *(core)* [**breaking**] make Chat append messages to caller history ([#1733](https://github.com/wey-gu/rig/pull/1733)) (by @gold-silver-copper)
- http client generic parameter part 2 ([#1698](https://github.com/wey-gu/rig/pull/1698)) (by @gold-silver-copper) - #1698
- http client generic parameter ([#1697](https://github.com/wey-gu/rig/pull/1697)) (by @FayCarsons) - #1697
- *(telemetry)* emit OTel-standard gen_ai.usage.cache_read.input_tokens across providers ([#1666](https://github.com/wey-gu/rig/pull/1666)) (by @alwayys-afk)
- *(bedrock)* preserve adaptive-thinking signatures in streaming reasoning ([#1683](https://github.com/wey-gu/rig/pull/1683)) (by @byQuexo)
- pass generic parameter to gemini capability types ([#1687](https://github.com/wey-gu/rig/pull/1687)) (by @FayCarsons) - #1687
- *(bedrock)* handle adaptive-thinking interactions for prompt caching and reasoning conversion ([#1675](https://github.com/wey-gu/rig/pull/1675)) (by @byQuexo)
- protoc CI ([#1676](https://github.com/wey-gu/rig/pull/1676)) (by @gold-silver-copper) - #1676
- *(bedrock)* preserve all parallel tool calls in completion response ([#1626](https://github.com/wey-gu/rig/pull/1626)) (by @aleksmeshr)
- *(openai)* carry reasoning_content on assistant tool-call messages ([#1649](https://github.com/wey-gu/rig/pull/1649)) (by @indrazm)
- preserve multimodal tool results in streaming chat history ([#1661](https://github.com/wey-gu/rig/pull/1661)) (by @gold-silver-copper) - #1661
- OpenAI text extraction  ([#1660](https://github.com/wey-gu/rig/pull/1660)) (by @gold-silver-copper) - #1660
- fixed n tests ([#1659](https://github.com/wey-gu/rig/pull/1659)) (by @gold-silver-copper) - #1659
- *(rig-1283)* handle llama.cpp reasoning_content as content ([#1657](https://github.com/wey-gu/rig/pull/1657)) (by @inqode-lars)
- fix and example ([#1646](https://github.com/wey-gu/rig/pull/1646)) (by @gold-silver-copper) - #1646
- *(responses_api)* add Unknown catch-all variant to Output enum ([#1552](https://github.com/wey-gu/rig/pull/1552)) (by @BillionClaw)
- *(#1604)* delay response_format on initial tool turns  (#1622) (by @gold-silver-copper)
- reduce `ToolServer` contention during tool lookup and execution ([#1607](https://github.com/wey-gu/rig/pull/1607)) (by @isSerge) - #1607
- *(streaming)* preserve tool call history, deduplicate prompt ([#1590](https://github.com/wey-gu/rig/pull/1590)) (by @gold-silver-copper)
- *(openai)* capture ResponseFailed errors in stream mode ([#1582](https://github.com/wey-gu/rig/pull/1582)) (by @gabrielrondon)
- *(gemini)* infer string type for enum schemas in anyOf/oneOf ([#1547](https://github.com/wey-gu/rig/pull/1547)) (by @CMLKevin)
- include assistant text in chat_history during multi-turn streaming ([#1560](https://github.com/wey-gu/rig/pull/1560)) (by @dgrijalva) - #1560
- *(rig-derive)* propagate function visibility to generated tool structs ([#1570](https://github.com/wey-gu/rig/pull/1570)) (by @tomasz-feliksik)
- skip serializing encrypted_content when None ([#1534](https://github.com/wey-gu/rig/pull/1534)) (by @BillionClaw) - #1534
- *(openai)* make strict field optional in StructuredOutputsInput ([#1528](https://github.com/wey-gu/rig/pull/1528)) (by @BillionClaw)
- *(llamafile)* apply embedding Number->f64 conversion for arbitrary_precision compat ([#1526](https://github.com/wey-gu/rig/pull/1526)) (by @EinarasGar)
- embedding deserialization breaks with serde_json/arbitrary_precision ([#1518](https://github.com/wey-gu/rig/pull/1518)) (by @EinarasGar) - #1518
- *(openai)* strengthen streaming tool call dedup to prevent false evictions ([#1510](https://github.com/wey-gu/rig/pull/1510)) (by @majiayu000)
- *(gemini)* [**breaking**] resolve embedding dimensions dynamically instead of hardcoding ([#1513](https://github.com/wey-gu/rig/pull/1513)) (by @McTr0)
- release-plz config ([#1524](https://github.com/wey-gu/rig/pull/1524)) (by @joshua-mo-143) - #1524
- *(gemini)* support URL-backed text documents ([#1507](https://github.com/wey-gu/rig/pull/1507)) (by @joshua-mo-143)
- forward max_tokens in Chat Completions API requests ([#1495](https://github.com/wey-gu/rig/pull/1495)) (by @sonicxml) - #1495
- *(rig-1486)* refactor surrealdb to v3.0.2 ([#1487](https://github.com/wey-gu/rig/pull/1487)) (by @itsezc)
- populate cached_input_tokens in Chat Completions streaming ([#1485](https://github.com/wey-gu/rig/pull/1485)) (by @sonicxml) - #1485
- *(gemini)* correct ProviderBuilder impl for GeminiInteractionsBuilder ([#1482](https://github.com/wey-gu/rig/pull/1482)) (by @adrianncovaci)
- *(rig-1218)* gemini MCP tool invalid tool argument ([#1462](https://github.com/wey-gu/rig/pull/1462)) (by @joshua-mo-143)
- amend release-plz header/body ([#1474](https://github.com/wey-gu/rig/pull/1474)) (by @joshua-mo-143) - #1474
- *(rig-1210)* deepseek content should not split into separate messages ([#1460](https://github.com/wey-gu/rig/pull/1460)) (by @joshua-mo-143)
- *(rig-1209)* reasoning content dropped from deepseektool messages ([#1459](https://github.com/wey-gu/rig/pull/1459)) (by @joshua-mo-143)
- *(openai)* add reasoning_content to StreamingDelta for OpenAI-compatible providers ([#1441](https://github.com/wey-gu/rig/pull/1441)) (by @Fromsko)
- properly support PDF doc URLs (anthropic) ([#1431](https://github.com/wey-gu/rig/pull/1431)) (by @joshua-mo-143) - #1431
- URL doc returns HTTP 400 (OpenAI) ([#1432](https://github.com/wey-gu/rig/pull/1432)) (by @joshua-mo-143) - #1432
- `total_usage` -> `usage` ([#1453](https://github.com/wey-gu/rig/pull/1453)) (by @0xMochan) - #1453
- *(deps)* enable reqwest system-proxy for proxy env var support ([#1442](https://github.com/wey-gu/rig/pull/1442)) (by @Phoenix500526)
- *(streaming)* disambiguate tool calls sharing the same index from API gateways ([#1443](https://github.com/wey-gu/rig/pull/1443)) (by @Phoenix500526)
- allow empty arguments openrouter ([#1438](https://github.com/wey-gu/rig/pull/1438)) (by @CremboC) - #1438
- *(example/surrealdb)* calculate threshold from results ([#1422](https://github.com/wey-gu/rig/pull/1422)) (by @micllam)
- *(rig-1195)* image urls don't work with anthropic ([#1403](https://github.com/wey-gu/rig/pull/1403)) (by @joshua-mo-143)
- *(agents)* correct prompt hook docs, split modules, and fix install script ([#1384](https://github.com/wey-gu/rig/pull/1384)) (by @illegalcall)
- fix ollama dims miss ([#1199](https://github.com/wey-gu/rig/pull/1199)) (by @KanonWY) - #1199
- *(rig-1182)* assistantcontent serialization when empty (openai) ([#1369](https://github.com/wey-gu/rig/pull/1369)) (by @joshua-mo-143)
- *(rig-1183)* invalid options provided (ollama) ([#1365](https://github.com/wey-gu/rig/pull/1365)) (by @joshua-mo-143)
- *(rig-1178)* not all responses APIs support instructions ([#1347](https://github.com/wey-gu/rig/pull/1347)) (by @joshua-mo-143)
- avoid duplicate role in responses input ([#1314](https://github.com/wey-gu/rig/pull/1314)) (by @acgxv) - #1314
- *(providers)* fixed azure openai embedding dimension ([#1303](https://github.com/wey-gu/rig/pull/1303)) (by @austinsimpsond41)
- *(rig-1174)* openai responses requires reasoning in history ([#1335](https://github.com/wey-gu/rig/pull/1335)) (by @joshua-mo-143)
- *(rig-1170)* concurrent tool execution ([#1326](https://github.com/wey-gu/rig/pull/1326)) (by @joshua-mo-143)
- *(rig-1167)* fix deepseek-reasoner v3.2 invoke ([#1333](https://github.com/wey-gu/rig/pull/1333)) (by @chenyh1990)
- *(rig-1156)* impl VectorStoreIndexDyn for mongodb and milvus ([#1300](https://github.com/wey-gu/rig/pull/1300)) (by @joshua-mo-143)
- *(rig-1154)* gemini API tools mismatch ([#1291](https://github.com/wey-gu/rig/pull/1291)) (by @joshua-mo-143)
- *(providers)* re-export gemini EmbeddingModel and constants at module root ([#1292](https://github.com/wey-gu/rig/pull/1292)) (by @martsokha)
- *(agent)* fix CancelSignal cancellation and reason sharing bugs ([#1282](https://github.com/wey-gu/rig/pull/1282)) (by @yexiyue)
- *(rig-1140)* do not prepend a forward slash to blank base URLs ([#1275](https://github.com/wey-gu/rig/pull/1275)) (by @joshua-mo-143)
- some completion providers send usage chunks with 0 completion choices causing 0 reported usage ([#1211](https://github.com/wey-gu/rig/pull/1211)) (by @dxvid-pts) - #1211
- *(rig-1118)* rig-sqlite incorrectly uses euclidian distance ([#1217](https://github.com/wey-gu/rig/pull/1217)) (by @joshua-mo-143)
- *(rig-1109)* export agent StreamingResult type ([#1220](https://github.com/wey-gu/rig/pull/1220)) (by @joshua-mo-143)
- docs typo ([#1219](https://github.com/wey-gu/rig/pull/1219)) (by @joshua-mo-143) - #1219
- missing json header on send_streaming ([#1196](https://github.com/wey-gu/rig/pull/1196)) (by @Sytten) - #1196
- *(rig-1113)* `calculate_max_tokens` assumes known model (anthropic) ([#1216](https://github.com/wey-gu/rig/pull/1216)) (by @joshua-mo-143)
- readme header spacing ([#1181](https://github.com/wey-gu/rig/pull/1181)) (by @mateobelanger) - #1181
- add headers to get call ([#1178](https://github.com/wey-gu/rig/pull/1178)) (by @Sytten) - #1178
- deepseek stream_prompt Invalid status code 415 ([#1170](https://github.com/wey-gu/rig/pull/1170)) (by @gj-zhang) - #1170
- *(openrouter)* add default serde attr to reasoning_details for optional field deserialization ([#1173](https://github.com/wey-gu/rig/pull/1173)) (by @adrianncovaci)
- *(groq)* rename StreamingOptions to StreamOptions ([#1159](https://github.com/wey-gu/rig/pull/1159)) (by @yazaldefilimone)
- *(openai)* add None variant to ReasoningEffort enum ([#1158](https://github.com/wey-gu/rig/pull/1158)) (by @vandenbogart)
- StreamingPromptHook panic when chat_history is empty ([#1132](https://github.com/wey-gu/rig/pull/1132)) (by @iskng) - #1132
- *(rig-1087)* groq should accept tool calls ([#1137](https://github.com/wey-gu/rig/pull/1137)) (by @joshua-mo-143)
- *(rig-1082)* unnecessary JSON flatten ([#1127](https://github.com/wey-gu/rig/pull/1127)) (by @joshua-mo-143)
- OpenAI Chat Completions API tool result format and strict mode support ([#1117](https://github.com/wey-gu/rig/pull/1117)) (by @lleukkun) - #1117
- *(rig-1081)* oneOf not supported by OpenAI tool calling ([#1124](https://github.com/wey-gu/rig/pull/1124)) (by @joshua-mo-143)
- *(streaming)* use .instrument() instead of span.enter() to prevent span leak ([#1108](https://github.com/wey-gu/rig/pull/1108)) (by @heeen)
- *(rig-1093)* gemini config error when no additional params used ([#1094](https://github.com/wey-gu/rig/pull/1094)) (by @joshua-mo-143)
- OpenAI required props for structured output ([#1090](https://github.com/wey-gu/rig/pull/1090)) (by @TheoOiry) - #1090
- *(rig-1055)* remove deprecated gemini-2.5-flash preview ([#1084](https://github.com/wey-gu/rig/pull/1084)) (by @joshua-mo-143)
- rmcp derive clone ([#1080](https://github.com/wey-gu/rig/pull/1080)) (by @edisontim) - #1080
- *(rig-1050)* Inconsistent model/agent initialisation methods ([#1069](https://github.com/wey-gu/rig/pull/1069)) (by @FayCarsons)
- *(gemini-request)* add `#[serde(default)]` for missing `generation_config` field ([#1060](https://github.com/wey-gu/rig/pull/1060)) (by @Numeration)
- handle errors in document serialization and upsert ([#1047](https://github.com/wey-gu/rig/pull/1047)) (by @cody-why) - #1047
- update imported packages in the code example ([#1041](https://github.com/wey-gu/rig/pull/1041)) (by @jaybe1234) - #1041
- *(rig-1035)* export StreamingPromptHook ([#1039](https://github.com/wey-gu/rig/pull/1039)) (by @joshua-mo-143)
- Gemini responses lacking content ([#1030](https://github.com/wey-gu/rig/pull/1030)) (by @ribelo) - #1030
- *(rig-1029)* Reasoning not handled properly for agent stream prompt ([#1024](https://github.com/wey-gu/rig/pull/1024)) (by @joshua-mo-143)
- *(openai-responses)* add `#[serde(default)]` for missing `tools` field ([#1021](https://github.com/wey-gu/rig/pull/1021)) (by @windoze)
- *(rig-1027)* allow any error type to be used for rig tool macro ([#1017](https://github.com/wey-gu/rig/pull/1017)) (by @joshua-mo-143)
- implement Serialize for LanceDBFilter to enable LanceDbVectorIndex usage in dynamic_context ([#1010](https://github.com/wey-gu/rig/pull/1010)) (by @cody-why) - #1010
- compliance with OpenAI API  stream error "message":"Model field is required." ([#1006](https://github.com/wey-gu/rig/pull/1006)) (by @cody-why) - #1006
- CompletionError: ProviderError: {"error":{"code":null,"param":null,"message":"[] is too short - 'tools'","type":"invalid_request_error"}} ([#1003](https://github.com/wey-gu/rig/pull/1003)) (by @cody-why) - #1003
- *(rig-1023)* reasoning/thinking stream sends redundant data ([#1002](https://github.com/wey-gu/rig/pull/1002)) (by @joshua-mo-143)
- *(rig-1022)* GenericEventSource polling None should not error ([#999](https://github.com/wey-gu/rig/pull/999)) (by @joshua-mo-143)
- *(huggingface)* align tool message serialization with OpenAI API spec ([#993](https://github.com/wey-gu/rig/pull/993)) (by @hadronzoo)
- *(rig-1020)* add `futures-timer/wasm-bindgen` feature for wasm ([#995](https://github.com/wey-gu/rig/pull/995)) (by @joshua-mo-143)
- *(rig-1019)* fix potentially incorrect provider URLs ([#991](https://github.com/wey-gu/rig/pull/991)) (by @joshua-mo-143)
- *(rig-1016)* Huggingface completions API 404 ([#986](https://github.com/wey-gu/rig/pull/986)) (by @joshua-mo-143)
- *(rig-1011)* docs mismatch ([#981](https://github.com/wey-gu/rig/pull/981)) (by @joshua-mo-143)
- *(rig-1007)* tool servers broken in WASM ([#970](https://github.com/wey-gu/rig/pull/970)) (by @joshua-mo-143)
- *(rig-1009)* Incorrect struct shape (OpenAI) ([#973](https://github.com/wey-gu/rig/pull/973)) (by @joshua-mo-143)
- *(rig-997)* allow string documents for OpenAI Completions API ([#966](https://github.com/wey-gu/rig/pull/966)) (by @joshua-mo-143)
- *(rig-1006)* text-embedding-ada-002 doesn't support custom dimensions ([#967](https://github.com/wey-gu/rig/pull/967)) (by @joshua-mo-143)
- *(agent)* Apply tool_choice to completion request ([#958](https://github.com/wey-gu/rig/pull/958)) (by @sinkingsugar)
- *(rig-1005)* enable toggling "think" on ollama ([#962](https://github.com/wey-gu/rig/pull/962)) (by @joshua-mo-143)
- *(openrouter)* use reqwest_post helper to construct full URL ([#943](https://github.com/wey-gu/rig/pull/943)) (by @adrianncovaci)
- *(rig-995)* include max tokens in Moonshot API request ([#935](https://github.com/wey-gu/rig/pull/935)) (by @joshua-mo-143)
- *(rig-991)* nested struct conversion to Gemini OpenAPI type schema ([#926](https://github.com/wey-gu/rig/pull/926)) (by @joshua-mo-143)
- trying to fix test regressions part 2 ([#913](https://github.com/wey-gu/rig/pull/913)) (by @joshua-mo-143) - #913
- test regression ([#912](https://github.com/wey-gu/rig/pull/912)) (by @joshua-mo-143) - #912
- *(rig-982)* embedding_model_with_ndims() doesn't pass dimensions parameter to OpenAI API (by @pushkarm029)
- *(rig-983)* http request fail due to no content type header set ([#909](https://github.com/wey-gu/rig/pull/909)) (by @joshua-mo-143)
- *(rig-980)* cosine similarity threshold should work (helixdb) ([#899](https://github.com/wey-gu/rig/pull/899)) (by @joshua-mo-143)
- Correct data structure for OpenAI responses images and PDFs ([#880](https://github.com/wey-gu/rig/pull/880)) (by @z0w0) - #880
- only youtube videos should accept null mime type (gemini) ([#873](https://github.com/wey-gu/rig/pull/873)) (by @joshua-mo-143) - #873
- *(rig-970)* file URLs should be able to accept empty media type (Gemini) ([#872](https://github.com/wey-gu/rig/pull/872)) (by @joshua-mo-143)
- *(rig-970)* youtube video ingestion doesn't work (gemini) (by @joshua-mo-143)
- fix(rig-962)(deepseek): tool calls not recognised when put behind text content ([#862](https://github.com/wey-gu/rig/pull/862)) (by @joshua-mo-143) - #862
- fix-853 ([#854](https://github.com/wey-gu/rig/pull/854)) (by @danparizher) - #854
- *(rig-956)* DocumentSourceKind fails to serialize with common serializers ([#849](https://github.com/wey-gu/rig/pull/849)) (by @joshua-mo-143)
- *(rig-957)* huggingface should convert image URLs ([#848](https://github.com/wey-gu/rig/pull/848)) (by @joshua-mo-143)
- *(rig-950)* openai imagegen doesn't work with gpt-image-1 ([#837](https://github.com/wey-gu/rig/pull/837)) (by @joshua-mo-143)
- gh workflow files ([#835](https://github.com/wey-gu/rig/pull/835)) (by @joshua-mo-143) - #835
- ci lints ([#832](https://github.com/wey-gu/rig/pull/832)) (by @joshua-mo-143) - #832
- *(rig-944)* surrealdb WHERE clause causes no results ([#821](https://github.com/wey-gu/rig/pull/821)) (by @joshua-mo-143)
- *(rig-939)* incomplete byte sequence error when streaming from OpenAI Responses ([#812](https://github.com/wey-gu/rig/pull/812)) (by @joshua-mo-143)
- *(rig-933)* openai responses api integration does not properly take images ([#799](https://github.com/wey-gu/rig/pull/799)) (by @joshua-mo-143)
- rig-qdrant docs don't compile due to feature being added ([#787](https://github.com/wey-gu/rig/pull/787)) ([#794](https://github.com/wey-gu/rig/pull/794)) (by @arhangelskij) - #794
- *(rig-core examples)* add `required` field to calculator example tool definitions ([#757](https://github.com/wey-gu/rig/pull/757)) (by @FayCarsons)
- *(openai responses)* recursively add additionalProperties: false to nested schemas ([#755](https://github.com/wey-gu/rig/pull/755)) (by @Soroushsrd)
- empty type in Vec<T> schema conversion for Gemini API ([#721](https://github.com/wey-gu/rig/pull/721)) ([#748](https://github.com/wey-gu/rig/pull/748)) (by @Soroushsrd) - #748
- docs are broken (...again) ([#722](https://github.com/wey-gu/rig/pull/722)) (by @joshua-mo-143) - #722
- *(rig-890)* docs are broken ([#718](https://github.com/wey-gu/rig/pull/718)) (by @joshua-mo-143)
- *(rig-886)* only GenerationConfig can be passed into additional_params ([#707](https://github.com/wey-gu/rig/pull/707)) (by @joshua-mo-143)
- deepseek streaming endpoint ([#687](https://github.com/wey-gu/rig/pull/687)) (by @appflowy) - #687
- *(rig-864)* missing id from OpenAI Responses API for reasoning items ([#681](https://github.com/wey-gu/rig/pull/681)) (by @joshua-mo-143)
- *(rig-853)* gemini streaming impl ignores reasoning chunks ([#654](https://github.com/wey-gu/rig/pull/654)) (by @joshua-mo-143)
- Ollama provider handling of canonical URLs ([#656](https://github.com/wey-gu/rig/pull/656)) (by @vrurg) - #656
- *(rig-852)* dynamic context does not work correctly with ollama ([#660](https://github.com/wey-gu/rig/pull/660)) (by @joshua-mo-143)
- *(rig-824)* ToolResultContent should be serde-tagged ([#621](https://github.com/wey-gu/rig/pull/621)) (by @joshua-mo-143)
- *(rig-828)* support done message on openai streaming completions api ([#619](https://github.com/wey-gu/rig/pull/619)) (by @joshua-mo-143)
- *(rig-827)* openai responses streaming api placeholder panic ([#620](https://github.com/wey-gu/rig/pull/620)) (by @joshua-mo-143)
- *(rig-834)* erroeneous tracing log level ([#626](https://github.com/wey-gu/rig/pull/626)) (by @joshua-mo-143)
- *(rig-820)* ensure call ID is properly propagated ([#601](https://github.com/wey-gu/rig/pull/601)) (by @joshua-mo-143)
- *(rig-815)* gemini completion fails when used with no tools ([#589](https://github.com/wey-gu/rig/pull/589)) (by @joshua-mo-143)
- *(rig-811)* ollama fails to return results from multiple tools ([#581](https://github.com/wey-gu/rig/pull/581)) (by @joshua-mo-143)
- *(rig-810)* prompting OpenAI reponses with message history fails ([#578](https://github.com/wey-gu/rig/pull/578)) (by @joshua-mo-143)
- *(rig-809)* gemini function declarations should not be OneOrMany ([#576](https://github.com/wey-gu/rig/pull/576)) (by @joshua-mo-143)
- dependencies ([#572](https://github.com/wey-gu/rig/pull/572)) (by @joshua-mo-143) - #572
- actually fix ci this time (hopefully) ([#571](https://github.com/wey-gu/rig/pull/571)) (by @joshua-mo-143) - #571
- *(rig-808)* fix CD pipeline ([#570](https://github.com/wey-gu/rig/pull/570)) (by @joshua-mo-143)
- Retain multi-turn tool call results in case of response error ([#526](https://github.com/wey-gu/rig/pull/526)) (by @wonrax) - #526
- *(rig-794)* parse openAI SSE response error ([#545](https://github.com/wey-gu/rig/pull/545)) (by @joshua-mo-143)
- *(rig-796)* OpenRouter extractor fails ([#544](https://github.com/wey-gu/rig/pull/544)) (by @joshua-mo-143)
- *(rig-792)* inconsistent implementations of with_custom_client ([#530](https://github.com/wey-gu/rig/pull/530)) (by @joshua-mo-143)
- *(rig-783)* tool call example doesn't work with Gemini and OpenRouter ([#515](https://github.com/wey-gu/rig/pull/515)) (by @joshua-mo-143)
- *(rig-787)* discrepancy in rust crate editions ([#519](https://github.com/wey-gu/rig/pull/519)) (by @joshua-mo-143)
- *(rig-773)* xAI embeddings endpoint is wrong ([#492](https://github.com/wey-gu/rig/pull/492)) (by @joshua-mo-143)
- OpenAI provider streaming tool call response for local LLM ([#442](https://github.com/wey-gu/rig/pull/442)) (by @kosz78) - #442
- *(rig-761)* ollama drops tool call results ([#478](https://github.com/wey-gu/rig/pull/478)) (by @joshua-mo-143)
- Update of xAI model list ([#486](https://github.com/wey-gu/rig/pull/486)) (by @ProHaller) - #486
- *(rig-757)* CI fails because of new clippy lints ([#470](https://github.com/wey-gu/rig/pull/470)) (by @joshua-mo-143)
- *(extractor)* correct typo in extractor prompt ([#460](https://github.com/wey-gu/rig/pull/460)) (by @QwF8PatkuU)
- *(message)* correct ToolCall to Message conversion ([#461](https://github.com/wey-gu/rig/pull/461)) (by @QwF8PatkuU)
- Fix `dims` value for gemini's `EMBEDDING_004` ([#452](https://github.com/wey-gu/rig/pull/452)) ([#453](https://github.com/wey-gu/rig/pull/453)) (by @adrianncovaci) - #453
- bump mcp-core to latest version and fixed breaking changes ([#443](https://github.com/wey-gu/rig/pull/443)) (by @mkranjac) - #443
- fixed bug with base64 encoding on AWS Bedrock ([#432](https://github.com/wey-gu/rig/pull/432)) (by @mkranjac) - #432
- system and developer messages for openai ([#430](https://github.com/wey-gu/rig/pull/430)) (by @0xMochan) - #430
- update broken link  ([#429](https://github.com/wey-gu/rig/pull/429)) (by @PixelPil0t1) - #429
- o-series models + constants ([#426](https://github.com/wey-gu/rig/pull/426)) (by @0xMochan) - #426
- dynamically pull rag text from chat history ([#425](https://github.com/wey-gu/rig/pull/425)) (by @0xMochan) - #425
- rig tool macro struct not public ([#409](https://github.com/wey-gu/rig/pull/409)) (by @joshua-mo-143) - #409
- function call conversion typo ([#415](https://github.com/wey-gu/rig/pull/415)) (by @jizizr) - #415
- deepseek function call conversion typo ([#414](https://github.com/wey-gu/rig/pull/414)) (by @joshua-mo-143) - #414
- gemini embeddings does not work for multiple documents ([#386](https://github.com/wey-gu/rig/pull/386)) (by @joshua-mo-143) - #386
- deserialization error due to serde rename of tool result ([#374](https://github.com/wey-gu/rig/pull/374)) (by @joshua-mo-143) - #374
- no params tools definition for Gemini ([#363](https://github.com/wey-gu/rig/pull/363)) (by @piotrostr) - #363
- contributing.md link (by @futreall) - #361
- *(openai)* serde rename for image_url UserContent ([#355](https://github.com/wey-gu/rig/pull/355)) (by @joshua-mo-143)
- unnecessary `unwrap`, skip serializing empty vec ([#343](https://github.com/wey-gu/rig/pull/343)) (by @M4n5ter) - #343
- fix error handling for Qwen's responses when using tools ([#351](https://github.com/wey-gu/rig/pull/351)) (by @qhliang) - #351
- reqwest can not use SOCKS proxy ([#311](https://github.com/wey-gu/rig/pull/311))
- fix wrong debug message ([#342](https://github.com/wey-gu/rig/pull/342)) (by @M4n5ter) - #342
- *(openai)* skip serializing empty tool_calls vector ([#327](https://github.com/wey-gu/rig/pull/327)) (by @GustavoWidman)
- *(surrealdb-rig)* Cargo.toml info ([#319](https://github.com/wey-gu/rig/pull/319)) (by @joshua-mo-143)
- *(openai)* correct some fields for tools ([#286](https://github.com/wey-gu/rig/pull/286)) (by @s6nqou)
- *(loaders)* bump lodpf to allow more PDFs to parse correctly ([#307](https://github.com/wey-gu/rig/pull/307)) (by @lispking)
- xai agent prompt provider error ([#305](https://github.com/wey-gu/rig/pull/305)) ([#306](https://github.com/wey-gu/rig/pull/306)) (by @obikata) - #306
- enhance tracing messages ([#287](https://github.com/wey-gu/rig/pull/287)) (by @threewebcode)
- *(gemini)* fixed tool calling + tool extractor demo ([#297](https://github.com/wey-gu/rig/pull/297)) (by @0xMochan)
- o3-mini doesn't support temperature ([#266](https://github.com/wey-gu/rig/pull/266)) (by @joshua-mo-143) - #266
- *(rig-fastembed)* crate info ([#294](https://github.com/wey-gu/rig/pull/294)) (by @joshua-mo-143)
- deepseek client auth ([#279](https://github.com/wey-gu/rig/pull/279)) (by @carlos-verdes) - #279
- *(galadriel)* missed fixes from messages pr ([#270](https://github.com/wey-gu/rig/pull/270)) (by @0xMochan)
- mongodb vector search example (by @dandagow3t) - #217
- *(providers)* provider wasm support ([#245](https://github.com/wey-gu/rig/pull/245)) (by @cvauclair)
- Use of deprecated `prelude` module ([#241](https://github.com/wey-gu/rig/pull/241)) (by @cvauclair) - #241
- anthropic tool use ([#168](https://github.com/wey-gu/rig/pull/168)) (by @edisontim) - #168
- *(rig-sqlite)* Use tokio-rusqlite from crates.io instead of custom crate ([#158](https://github.com/wey-gu/rig/pull/158)) (by @edisontim)
- *(example)* ollama example uses wrong url (by @wale-e)
- *(openai)* Make integration more general ([#156](https://github.com/wey-gu/rig/pull/156)) (by @M4n5ter)
- rig-derive version ([#138](https://github.com/wey-gu/rig/pull/138)) (by @cvauclair) - #138
- rig-sqlite missing version in Cargo.toml ([#137](https://github.com/wey-gu/rig/pull/137)) (by @cvauclair) - #137
- *(rig-sqlite)* Fix missing rig-core version (by @cvauclair)
- *(neo4j)* remove embeddings from top_n lookup ([#118](https://github.com/wey-gu/rig/pull/118)) (by @mateobelanger)
- remove unused module ([#132](https://github.com/wey-gu/rig/pull/132)) (by @cvauclair) - #132
- *(readme)* perplexity logo size (by @mateobelanger)
- *(rig-mongodb)* remove embeddings from `top_n` lookup ([#115](https://github.com/wey-gu/rig/pull/115)) (by @0xMochan)
- *(rig-lancedb)* rag embedding filtering ([#104](https://github.com/wey-gu/rig/pull/104)) (by @marieaurore123)
- *(rig-qdrant)* Rig Qdrant dependency ([#98](https://github.com/wey-gu/rig/pull/98)) (by @cvauclair)
- wrong reference to companion crate (by @mateobelanger) - #79
- missing qdrant readme reference (by @mateobelanger) - #79
- outdated example and broken link (by @mateobelanger) - #79
- *(gemini)* issue when additionnal param is empty (by @mateobelanger)
- docs imports and refs (by @mateobelanger) - #56
- *(gemini)* missing param to be marked as optional in completion res (by @mateobelanger)
- *(vector-index)* In memory vector store index incorrect search (by @cvauclair)
- update lancedb examples test data (by @marieaurore123) - #33
- *(ci)* install protobuf-compiler in test  job (by @marieaurore123)
- make PR changes Pt II (by @marieaurore123) - #33
- make PR changes pt I (by @marieaurore123) - #33
- adjust version const naming (by @0xMochan) - #27
- implement review suggestions + renaming (by @0xMochan) - #27
- add `completion_request.documents` to `chat_history` (by @0xMochan) - #27
- adjust API to be cleaner + add docstrings (by @0xMochan) - #27
- *(perplexity)* fix preamble and context in completion request (by @marieaurore123)
- workflows permissions (by @cvauclair) - #10
- clippy warnings (by @cvauclair) - #7
- Add proper error models for LLM providers (by @cvauclair)

### Other

- *(rig-core)* [**breaking**] remove the experimental pipeline module ([#1941](https://github.com/wey-gu/rig/pull/1941)) (by @gold-silver-copper)
- run doctests and stop rig-sqlite opting out of them ([#1939](https://github.com/wey-gu/rig/pull/1939)) (by @gold-silver-copper) - #1939
- *(rig-core)* replace nanoid with fastrand for internal IDs ([#1938](https://github.com/wey-gu/rig/pull/1938)) (by @gold-silver-copper)
- *(examples)* migrate to a package-per-example layout ([#1937](https://github.com/wey-gu/rig/pull/1937)) (by @gold-silver-copper)
- add Archestra to "Who is using Rig?" section ([#1925](https://github.com/wey-gu/rig/pull/1925)) (by @arsenyinfo) - #1925
- release v0.39.0 ([#1888](https://github.com/wey-gu/rig/pull/1888)) (by @github-actions[bot]) - #1888
- *(deps)* bump uuid from 1.23.1 to 1.23.3 ([#1907](https://github.com/wey-gu/rig/pull/1907)) (by @dependabot[bot])
- *(deps)* bump lopdf from 0.40.0 to 0.41.0 ([#1877](https://github.com/wey-gu/rig/pull/1877)) (by @dependabot[bot])
- *(deps)* bump http from 1.4.0 to 1.4.2 ([#1909](https://github.com/wey-gu/rig/pull/1909)) (by @dependabot[bot])
- *(deps)* bump futures-timer from 3.0.3 to 3.0.4 ([#1908](https://github.com/wey-gu/rig/pull/1908)) (by @dependabot[bot])
- *(examples)* add Gemini mid-stream disruption token-counting example ([#1918](https://github.com/wey-gu/rig/pull/1918)) (by @gold-silver-copper)
- *(tool)* back ToolSet with an IndexMap instead of HashMap + order Vec ([#1916](https://github.com/wey-gu/rig/pull/1916)) (by @gold-silver-copper)
- de-flake tracing span tests and deepseek permission_control race ([#1915](https://github.com/wey-gu/rig/pull/1915)) (by @gold-silver-copper) - #1915
- *(agent)* cassette-backed AgentRun coverage against real Gemini turns ([#1901](https://github.com/wey-gu/rig/pull/1901)) (by @gold-silver-copper)
- Fix streaming reasoning history order ([#1898](https://github.com/wey-gu/rig/pull/1898)) (by @gold-silver-copper) - #1898
- Fix context document ordering ([#1893](https://github.com/wey-gu/rig/pull/1893)) (by @gold-silver-copper) - #1893
- Point ecosystem link to awesome-rig ([#1895](https://github.com/wey-gu/rig/pull/1895)) (by @gold-silver-copper) - #1895
- Add Gemini Nano Banana image generation ([#1889](https://github.com/wey-gu/rig/pull/1889)) (by @gold-silver-copper) - #1889
- release v0.38.2 ([#1854](https://github.com/wey-gu/rig/pull/1854)) (by @github-actions[bot]) - #1854
- *(deps)* bump tonic-prost-build from 0.14.5 to 0.14.6 ([#1874](https://github.com/wey-gu/rig/pull/1874)) (by @dependabot[bot])
- *(deps)* bump convert_case from 0.10.0 to 0.11.0 ([#1875](https://github.com/wey-gu/rig/pull/1875)) (by @dependabot[bot])
- *(deps)* bump reqwest from 0.13.3 to 0.13.4 ([#1873](https://github.com/wey-gu/rig/pull/1873)) (by @dependabot[bot])
- *(deps)* bump reqwest-middleware from 0.5.1 to 0.5.2 ([#1876](https://github.com/wey-gu/rig/pull/1876)) (by @dependabot[bot])
- Remove rig-redis integration ([#1887](https://github.com/wey-gu/rig/pull/1887)) (by @gold-silver-copper) - #1887
- migrate Copilot tests to cassette replay ([#1882](https://github.com/wey-gu/rig/pull/1882)) (by @gold-silver-copper) - #1882
- Redis vector store integration ([#1509](https://github.com/wey-gu/rig/pull/1509)) (by @daric93) - #1509
- add Ryzome to README nav links ([#1879](https://github.com/wey-gu/rig/pull/1879)) (by @mateobelanger) - #1879
- [codex] support mistral.rs OpenAI-compatible reasoning ([#1864](https://github.com/wey-gu/rig/pull/1864)) (by @gold-silver-copper) - #1864
- convert DeepSeek live tests to cassettes ([#1870](https://github.com/wey-gu/rig/pull/1870)) (by @gold-silver-copper) - #1870
- [codex] add OpenRouter cassette-backed provider coverage ([#1869](https://github.com/wey-gu/rig/pull/1869)) (by @gold-silver-copper) - #1869
- convert xAI live tests to cassettes ([#1868](https://github.com/wey-gu/rig/pull/1868)) (by @gold-silver-copper) - #1868
- [codex] cover Anthropic streaming tool result batching ([#1863](https://github.com/wey-gu/rig/pull/1863)) (by @gold-silver-copper) - #1863
- release ([#1851](https://github.com/wey-gu/rig/pull/1851)) (by @github-actions[bot]) - #1851
- unify workspace crate versions ([#1853](https://github.com/wey-gu/rig/pull/1853)) (by @gold-silver-copper) - #1853
- release ([#1765](https://github.com/wey-gu/rig/pull/1765)) (by @github-actions[bot]) - #1765
- Unify invalid tool call recovery with prompt hooks ([#1850](https://github.com/wey-gu/rig/pull/1850)) (by @gold-silver-copper) - #1850
- *(deps)* bump mongodb from 3.6.0 to 3.7.0 ([#1848](https://github.com/wey-gu/rig/pull/1848)) (by @dependabot[bot])
- *(deps)* bump zerocopy from 0.8.48 to 0.8.50 ([#1847](https://github.com/wey-gu/rig/pull/1847)) (by @dependabot[bot])
- *(deps)* bump google-cloud-aiplatform-v1 from 1.10.0 to 1.11.0 ([#1846](https://github.com/wey-gu/rig/pull/1846)) (by @dependabot[bot])
- *(deps)* bump serde_json from 1.0.149 to 1.0.150 ([#1845](https://github.com/wey-gu/rig/pull/1845)) (by @dependabot[bot])
- *(deps)* bump tonic from 0.14.5 to 0.14.6 ([#1844](https://github.com/wey-gu/rig/pull/1844)) (by @dependabot[bot])
- Fix parsing of streamed function-call argument deltas ([#1828](https://github.com/wey-gu/rig/pull/1828)) (by @geraschenko) - #1828
- *(deps)* port dependency bumps and Rust 1.91 ([#1842](https://github.com/wey-gu/rig/pull/1842)) (by @gold-silver-copper)
- *(deps)* bump quick-xml from 0.39.4 to 0.40.1 ([#1818](https://github.com/wey-gu/rig/pull/1818)) (by @dependabot[bot])
- *(deps)* bump google-cloud-auth from 1.9.0 to 1.10.0 ([#1817](https://github.com/wey-gu/rig/pull/1817)) (by @dependabot[bot])
- Stabilize MongoDB vector search test ([#1841](https://github.com/wey-gu/rig/pull/1841)) (by @gold-silver-copper) - #1841
- fix VT Code line grammar in README ([#1824](https://github.com/wey-gu/rig/pull/1824)) (by @Shaurya-Sethi) - #1824
- [codex] Validate model tool calls ([#1823](https://github.com/wey-gu/rig/pull/1823)) (by @gold-silver-copper) - #1823
- [codex] apply Anthropic cache control to tools ([#1815](https://github.com/wey-gu/rig/pull/1815)) (by @gold-silver-copper) - #1815
- *(deps)* bump tokio-tungstenite from 0.23.1 to 0.28.0 ([#1784](https://github.com/wey-gu/rig/pull/1784)) (by @dependabot[bot])
- *(deps)* bump rmcp from 1.6.0 to 1.7.0 ([#1783](https://github.com/wey-gu/rig/pull/1783)) (by @dependabot[bot])
- *(deps)* bump tokio from 1.52.1 to 1.52.3 ([#1782](https://github.com/wey-gu/rig/pull/1782)) (by @dependabot[bot])
- Expose per-completion-call usage in agent responses ([#1787](https://github.com/wey-gu/rig/pull/1787)) (by @gold-silver-copper) - #1787
- *(gemini)* add streaming metadata cassettes ([#1777](https://github.com/wey-gu/rig/pull/1777)) (by @gold-silver-copper)
- Add replayable provider cassette tests ([#1769](https://github.com/wey-gu/rig/pull/1769)) (by @gold-silver-copper) - #1769
- release ([#1692](https://github.com/wey-gu/rig/pull/1692)) (by @github-actions[bot]) - #1692
- Clean up root facade features and integration docs ([#1764](https://github.com/wey-gu/rig/pull/1764)) (by @gold-silver-copper) - #1764
- fix "a ancient" grammar in glarb-glarb sample text ([#1755](https://github.com/wey-gu/rig/pull/1755)) (by @abhicris) - #1755
- *(deps)* bump lopdf from 0.36.0 to 0.40.0 ([#1754](https://github.com/wey-gu/rig/pull/1754)) (by @dependabot[bot])
- *(deps)* bump quick-xml from 0.39.2 to 0.39.4 ([#1752](https://github.com/wey-gu/rig/pull/1752)) (by @dependabot[bot])
- *(deps)* bump tonic-build from 0.14.5 to 0.14.6 ([#1751](https://github.com/wey-gu/rig/pull/1751)) (by @dependabot[bot])
- Move reusable test doubles into rig_core::test_utils ([#1745](https://github.com/wey-gu/rig/pull/1745)) (by @gold-silver-copper) - #1745
- workspace and docs cleanup ([#1742](https://github.com/wey-gu/rig/pull/1742)) (by @gold-silver-copper) - #1742
- openrouter vars ([#1741](https://github.com/wey-gu/rig/pull/1741)) (by @gold-silver-copper) - #1741
- Add provider file ID support for document inputs ([#1740](https://github.com/wey-gu/rig/pull/1740)) (by @gold-silver-copper) - #1740
- add smoke test for completion across all Copilot models ([#1730](https://github.com/wey-gu/rig/pull/1730)) (by @BigtoC) - #1730
- bump dependencies ([#1728](https://github.com/wey-gu/rig/pull/1728)) (by @gold-silver-copper) - #1728
- remove needless files ([#1715](https://github.com/wey-gu/rig/pull/1715)) (by @gold-silver-copper) - #1715
- AGENTS.MD, CONTRIBUTING.MD, and docs ([#1714](https://github.com/wey-gu/rig/pull/1714)) (by @gold-silver-copper) - #1714
- Add Bedrock integration tests ([#1707](https://github.com/wey-gu/rig/pull/1707)) (by @gold-silver-copper) - #1707
- release-plz fix, instructions cleanup ([#1704](https://github.com/wey-gu/rig/pull/1704)) (by @gold-silver-copper) - #1704
- test cleanup ([#1703](https://github.com/wey-gu/rig/pull/1703)) (by @gold-silver-copper) - #1703
- add nitpicker to project showcase ([#1695](https://github.com/wey-gu/rig/pull/1695)) (by @arsenyinfo) - #1695
- improve project organization and create rig crate ([#1699](https://github.com/wey-gu/rig/pull/1699)) (by @gold-silver-copper) - #1699
- release ([#1631](https://github.com/wey-gu/rig/pull/1631)) (by @github-actions[bot]) - #1631
- first ([#1691](https://github.com/wey-gu/rig/pull/1691)) (by @gold-silver-copper) - #1691
- add ilert to Who is using Rig list ([#1690](https://github.com/wey-gu/rig/pull/1690)) (by @byQuexo) - #1690
- Update permission_control.rs ([#1678](https://github.com/wey-gu/rig/pull/1678)) (by @gold-silver-copper) - #1678
- cleanup ([#1677](https://github.com/wey-gu/rig/pull/1677)) (by @gold-silver-copper) - #1677
- Add clippy no panic lints ([#1663](https://github.com/wey-gu/rig/pull/1663)) (by @gold-silver-copper) - #1663
- openai chat completions ([#1655](https://github.com/wey-gu/rig/pull/1655)) (by @gold-silver-copper) - #1655
- manual tool call example ([#1643](https://github.com/wey-gu/rig/pull/1643)) (by @gold-silver-copper) - #1643
- Remove `RwLock` from immutable state and execute futures concurrently ([#1641](https://github.com/wey-gu/rig/pull/1641)) (by @isSerge) - #1641
- add Con to ecosystem ([#1640](https://github.com/wey-gu/rig/pull/1640)) (by @wey-gu) - #1640
- Add Serialize/Deserialize derives to CompletionRequest, PromptResponse, TypedPromptResponse ([#1637](https://github.com/wey-gu/rig/pull/1637)) (by @geraschenko) - #1637
- wasm compat for model lister ([#1638](https://github.com/wey-gu/rig/pull/1638)) (by @gold-silver-copper) - #1638
- standardize required fields handling across builders ([#1611](https://github.com/wey-gu/rig/pull/1611)) (by @isSerge) - #1611
- remove deprecated code ([#1633](https://github.com/wey-gu/rig/pull/1633)) (by @gold-silver-copper) - #1633
- release ([#1592](https://github.com/wey-gu/rig/pull/1592)) (by @github-actions[bot]) - #1592
- *(deps)* bump anyhow from 1.0.101 to 1.0.102 ([#1490](https://github.com/wey-gu/rig/pull/1490)) (by @dependabot[bot])
- *(deps)* bump deranged from 0.4.0 to 0.5.6 ([#1493](https://github.com/wey-gu/rig/pull/1493)) (by @dependabot[bot])
- *(deps)* bump futures from 0.3.31 to 0.3.32 ([#1624](https://github.com/wey-gu/rig/pull/1624)) (by @dependabot[bot])
- *(deps)* bump tokio from 1.49.0 to 1.51.1 ([#1625](https://github.com/wey-gu/rig/pull/1625)) (by @dependabot[bot])
- (refactor): replace legacy Anthropic constants  ([#1616](https://github.com/wey-gu/rig/pull/1616)) (by @gold-silver-copper) - #1616
- Add ModelLister for Ollama, Anthropic, Mistral, OpenAI, Gemini ([#1587](https://github.com/wey-gu/rig/pull/1587)) (by @LHelge) - #1587
- gpt image 1.5 ([#1543](https://github.com/wey-gu/rig/pull/1543)) (by @kevinastock) - #1543
- *(rig-core)* [**breaking**] migrate examples to integration tests ([#1603](https://github.com/wey-gu/rig/pull/1603)) (by @gold-silver-copper)
- Do not stringify strings during tool output ([#1608](https://github.com/wey-gu/rig/pull/1608)) (by @gold-silver-copper) - #1608
- *(rig-core)* upgrade rmcp integration to 1.3, gate tests ([#1596](https://github.com/wey-gu/rig/pull/1596)) (by @gold-silver-copper)
- Add support for prompt caching in rig-bedrock ([#1584](https://github.com/wey-gu/rig/pull/1584)) (by @marcbrooker) - #1584
- Update release-plz.toml ([#1581](https://github.com/wey-gu/rig/pull/1581)) (by @gold-silver-copper) - #1581
- release ([#1532](https://github.com/wey-gu/rig/pull/1532)) (by @github-actions[bot]) - #1532
- enable specifying native-tls instead of default rustls ([#1558](https://github.com/wey-gu/rig/pull/1558)) (by @demoray) - #1558
- Fix VoyageAI Usage deserialization failure on missing prompt_tokens ([#1568](https://github.com/wey-gu/rig/pull/1568)) (by @joaquinhuigomez) - #1568
- OTel GenAI semconv fix +  anthropic automatic prompt caching  ([#1572](https://github.com/wey-gu/rig/pull/1572)) (by @lgaches) - #1572
- *(gemini)* Make `prompt_token_count` optional in gemini response ([#1548](https://github.com/wey-gu/rig/pull/1548)) (by @marieaurore123)
- release ([#1489](https://github.com/wey-gu/rig/pull/1489)) (by @github-actions[bot]) - #1489
- Change preamble to system message internally ([#1527](https://github.com/wey-gu/rig/pull/1527)) (by @joshua-mo-143) - #1527
- *(rig-1234)* only show contributors on release ([#1523](https://github.com/wey-gu/rig/pull/1523)) (by @joshua-mo-143)
- fix link in rig-core README ([#1502](https://github.com/wey-gu/rig/pull/1502)) (by @waltronix) - #1502
- add "Rig in the wild" section to ecosystem.md ([#1498](https://github.com/wey-gu/rig/pull/1498)) (by @joshua-mo-143) - #1498
- Add ChatShell to project showcase ([#1488](https://github.com/wey-gu/rig/pull/1488)) (by @s3anw3) - #1488
- Feat/gemini interactions api ([#1230](https://github.com/wey-gu/rig/pull/1230)) (by @iskng) - #1230
- release ([#1414](https://github.com/wey-gu/rig/pull/1414)) (by @github-actions[bot]) - #1414
- prepare v0.32.0 release ([#1473](https://github.com/wey-gu/rig/pull/1473)) (by @joshua-mo-143) - #1473
- *(deps)* bump quick-xml from 0.38.4 to 0.39.2 ([#1466](https://github.com/wey-gu/rig/pull/1466)) (by @dependabot[bot])
- *(deps)* bump convert_case from 0.8.0 to 0.10.0 ([#1467](https://github.com/wey-gu/rig/pull/1467)) (by @dependabot[bot])
- *(deps)* bump chrono from 0.4.43 to 0.4.44 ([#1470](https://github.com/wey-gu/rig/pull/1470)) (by @dependabot[bot])
- *(rig-1220)* mark rig-eternalai deprecated ([#1472](https://github.com/wey-gu/rig/pull/1472)) (by @joshua-mo-143)
- *(deps)* bump testcontainers from 0.26.3 to 0.27.1 ([#1468](https://github.com/wey-gu/rig/pull/1468)) (by @dependabot[bot])
- *(rig-1219)* mark rig-wasm as unmaintained ([#1465](https://github.com/wey-gu/rig/pull/1465)) (by @joshua-mo-143)
- *(rig-1204)* add contributor list to release-plz ([#1461](https://github.com/wey-gu/rig/pull/1461)) (by @joshua-mo-143)
- *(rig-1200)* improve Client::builder DX ([#1436](https://github.com/wey-gu/rig/pull/1436)) (by @FayCarsons)
- *(docs)* fix rig-integrations link in README ([#1427](https://github.com/wey-gu/rig/pull/1427)) (by @ckaznable)
- Add nitpik to ecosystem list ([#1416](https://github.com/wey-gu/rig/pull/1416)) (by @joshua-mo-143) - #1416
- *(deps)* update rmcp types for v0.16 API compatibility ([#1410](https://github.com/wey-gu/rig/pull/1410)) (by @adrianncovaci)
- Add support for gRPC Gemini API ([#1232](https://github.com/wey-gu/rig/pull/1232)) (by @dxvid-pts) - #1232
- release ([#1353](https://github.com/wey-gu/rig/pull/1353)) (by @github-actions[bot]) - #1353
- Disable default features on aws-bedrock-runtime ([#1363](https://github.com/wey-gu/rig/pull/1363)) (by @Sagebati) - #1363
- add client builder test to all providers ([#1385](https://github.com/wey-gu/rig/pull/1385)) (by @hghalebi) - #1385
- add ironclaw ([#1400](https://github.com/wey-gu/rig/pull/1400)) (by @joshua-mo-143) - #1400
- typed reasoning content model ([#1395](https://github.com/wey-gu/rig/pull/1395)) (by @darinkishore) - #1395
- *(streaming)* return updated history in FinalResponse ([#1210](https://github.com/wey-gu/rig/pull/1210)) (by @Phoenix500526)
- replace release-plz.toml with update script ([#1381](https://github.com/wey-gu/rig/pull/1381)) (by @joshua-mo-143) - #1381
- version-group amendments ([#1380](https://github.com/wey-gu/rig/pull/1380)) (by @joshua-mo-143) - #1380
- *(rig-1168)* add release-plz.toml ([#1379](https://github.com/wey-gu/rig/pull/1379)) (by @joshua-mo-143)
- *(rig-1184)* remove AgentBuilderSimple ([#1368](https://github.com/wey-gu/rig/pull/1368)) (by @joshua-mo-143)
- propagate current span to tool call ([#1361](https://github.com/wey-gu/rig/pull/1361)) (by @redroy44) - #1361
- *(rig-1176)* unify prompt hook interfaces ([#1352](https://github.com/wey-gu/rig/pull/1352)) (by @joshua-mo-143)
- release ([#1288](https://github.com/wey-gu/rig/pull/1288)) (by @github-actions[bot]) - #1288
- *(rig-1179)* stop publishing rig-eternalai ([#1346](https://github.com/wey-gu/rig/pull/1346)) (by @joshua-mo-143)
- *(rig-1164)* rename max_depth & related to max_turns (BREAKING) ([#1323](https://github.com/wey-gu/rig/pull/1323)) (by @joshua-mo-143)
- remove unnecessary feature requirement for test ([#1341](https://github.com/wey-gu/rig/pull/1341)) (by @joshua-mo-143) - #1341
- add .direnv to gitignore ([#1342](https://github.com/wey-gu/rig/pull/1342)) (by @FayCarsons) - #1342
- *(rig-1169)* agent-aware docs ([#1325](https://github.com/wey-gu/rig/pull/1325)) (by @joshua-mo-143)
- Update ecosystem list in README.md ([#1246](https://github.com/wey-gu/rig/pull/1246)) (by @sopaco) - #1246
- *(rig-1157)* Update xAI to Responses API ([#1316](https://github.com/wey-gu/rig/pull/1316)) (by @FayCarsons)
- *(rig-1175)* update testcontainers ([#1339](https://github.com/wey-gu/rig/pull/1339)) (by @joshua-mo-143)
- *(deps)* bump surrealdb from 2.3.10 to 2.5.0 ([#1337](https://github.com/wey-gu/rig/pull/1337)) (by @dependabot[bot])
- *(rig-1162)* amend examples folder location ([#1324](https://github.com/wey-gu/rig/pull/1324)) (by @joshua-mo-143)
- *(rig-1171)* update ollama docs ([#1327](https://github.com/wey-gu/rig/pull/1327)) (by @joshua-mo-143)
- *(deps)* bump oneshot from 0.1.11 to 0.1.13 ([#1336](https://github.com/wey-gu/rig/pull/1336)) (by @dependabot[bot])
- *(rig-1163)* ollama stream tool calls get ignored ([#1309](https://github.com/wey-gu/rig/pull/1309)) (by @joshua-mo-143)
- Handle error for HTTP client response ([#1237](https://github.com/wey-gu/rig/pull/1237)) (by @davidgraeff) - #1237
- Add default type parameter T = reqwest::Client to ollama's EmbeddingModel for consistency with other providers ([#1293](https://github.com/wey-gu/rig/pull/1293)) (by @martsokha) - #1293
- release ([#1240](https://github.com/wey-gu/rig/pull/1240)) (by @github-actions[bot]) - #1240
- *(rig-1148)* improve azure module docs ([#1276](https://github.com/wey-gu/rig/pull/1276)) (by @joshua-mo-143)
- bump dependencies ([#1257](https://github.com/wey-gu/rig/pull/1257)) (by @ThomasMarches) - #1257
- *(rig-1145)* update code snippet ([#1268](https://github.com/wey-gu/rig/pull/1268)) (by @joshua-mo-143)
- fix gemini streaming ([#1262](https://github.com/wey-gu/rig/pull/1262)) (by @FayCarsons) - #1262
- Add Cortex Memory to ecosystem documentation ([#1245](https://github.com/wey-gu/rig/pull/1245)) (by @sopaco) - #1245
- *(1111)* fix JSON value binding in rig-postgres ([#1233](https://github.com/wey-gu/rig/pull/1233)) (by @FayCarsons)
- Add `AgentBuilder::tools` for adding static tools dynamically ([#1236](https://github.com/wey-gu/rig/pull/1236)) (by @silentEAG) - #1236
- *(rig-1125)* ecosystem.md file ([#1227](https://github.com/wey-gu/rig/pull/1227)) (by @joshua-mo-143)
- *(rig-core)* Fix gemini doc example with wrong imports ([#1238](https://github.com/wey-gu/rig/pull/1238)) (by @marc2332)
- release ([#1171](https://github.com/wey-gu/rig/pull/1171)) (by @github-actions[bot]) - #1171
- add tool name to tool call delta streaming events ([#1222](https://github.com/wey-gu/rig/pull/1222)) (by @bebiksior) - #1222
- *(rig-1104)* invalid integrations link ([#1215](https://github.com/wey-gu/rig/pull/1215)) (by @joshua-mo-143)
- *(rig-1114)* add list of PRs that will be rejected ([#1214](https://github.com/wey-gu/rig/pull/1214)) (by @joshua-mo-143)
- *(deps)* update rmcp dependency to 0.12.0 ([#1182](https://github.com/wey-gu/rig/pull/1182)) (by @ThomasMarches)
- update rig logo assets and notable production users ([#1180](https://github.com/wey-gu/rig/pull/1180)) (by @Tachikoma000) - #1180
- *(deps)* upgrade rmcp dependency to 0.11 ([#1165](https://github.com/wey-gu/rig/pull/1165)) (by @ThomasMarches)
- Respect custom http headers for outgoing client requests ([#1166](https://github.com/wey-gu/rig/pull/1166)) (by @simonjanssen) - #1166
- release ([#1150](https://github.com/wey-gu/rig/pull/1150)) (by @github-actions[bot]) - #1150
- ToolCall Signature and additional parameters ([#1154](https://github.com/wey-gu/rig/pull/1154)) (by @Sytten) - #1154
- fix incorrect variable name in AgentBuilder examples ([#1160](https://github.com/wey-gu/rig/pull/1160)) (by @Phoenix500526) - #1160
- *(rig-1085)* groq reasoning format ([#1151](https://github.com/wey-gu/rig/pull/1151)) (by @joshua-mo-143)
- *(rig-1031)* remove worker crate dep ([#1149](https://github.com/wey-gu/rig/pull/1149)) (by @joshua-mo-143)
- *(rig-1090)* crate re-org ([#1145](https://github.com/wey-gu/rig/pull/1145)) (by @joshua-mo-143)
- Fix Openrouter completion request ([#1141](https://github.com/wey-gu/rig/pull/1141)) (by @ronaldslc) - #1141
- *(rig-1060)* add generic multipart body ([#1133](https://github.com/wey-gu/rig/pull/1133)) (by @joshua-mo-143)
- *(rig-1080)* add Cairnify to Who's Using Rig list ([#1138](https://github.com/wey-gu/rig/pull/1138)) (by @joshua-mo-143)
- add cache: true to rust toolchain step ([#1140](https://github.com/wey-gu/rig/pull/1140)) (by @joshua-mo-143) - #1140
- Turn ToolResult into Tool message for Mistral ([#1130](https://github.com/wey-gu/rig/pull/1130)) (by @aleksmeshr) - #1130
- flatten display impl for tool call errors ([#1125](https://github.com/wey-gu/rig/pull/1125)) (by @joshua-mo-143) - #1125
- fix API key censoring ([#1128](https://github.com/wey-gu/rig/pull/1128)) (by @FayCarsons) - #1128
- deny `dbg!`, `todo!`, and `unimplemented!` in CI ([#1121](https://github.com/wey-gu/rig/pull/1121)) (by @FayCarsons) - #1121
- release ([#1113](https://github.com/wey-gu/rig/pull/1113)) (by @github-actions[bot]) - #1113
- *(rig-1077)* ensure log level enabled before logging messages ([#1114](https://github.com/wey-gu/rig/pull/1114)) (by @joshua-mo-143)
- *(rig-1078)* remove messages from span telemetry ([#1112](https://github.com/wey-gu/rig/pull/1112)) (by @joshua-mo-143)
- release ([#1044](https://github.com/wey-gu/rig/pull/1044)) (by @github-actions[bot]) - #1044
- add `Content-Type: application/json` to regular http requests ([#1106](https://github.com/wey-gu/rig/pull/1106)) (by @FayCarsons) - #1106
- Deprecate `DynClientBuilder` ([#1105](https://github.com/wey-gu/rig/pull/1105)) (by @FayCarsons) - #1105
- `client::Client` can leak api keys that have been inserted into its headers ([#1102](https://github.com/wey-gu/rig/pull/1102)) (by @FayCarsons) - #1102
- *(rig-1071)* remove outdated models ([#1096](https://github.com/wey-gu/rig/pull/1096)) (by @joshua-mo-143)
- *(rig-1068)* remove unused chatbot module ([#1092](https://github.com/wey-gu/rig/pull/1092)) (by @joshua-mo-143)
- Simple JSON passthrough unwrapper ([#1086](https://github.com/wey-gu/rig/pull/1086)) (by @Knight-Ops) - #1086
- *(rig-777)* proper request modelling for every provider ([#1067](https://github.com/wey-gu/rig/pull/1067)) (by @joshua-mo-143)
- *(deps)* upgrade `rmcp` ([#1079](https://github.com/wey-gu/rig/pull/1079)) (by @adrianncovaci)
- OpenAI parsing ([#1058](https://github.com/wey-gu/rig/pull/1058)) (by @Sytten) - #1058
- *(rig-1048)* gh merge queue ([#1064](https://github.com/wey-gu/rig/pull/1064)) (by @joshua-mo-143)
- *(rig-1046)* update list of who's using rig ([#1061](https://github.com/wey-gu/rig/pull/1061)) (by @joshua-mo-143)
- clean up provider code ([#1052](https://github.com/wey-gu/rig/pull/1052)) (by @Sytten) - #1052
- release ([#1009](https://github.com/wey-gu/rig/pull/1009)) (by @github-actions[bot]) - #1009
- make CompletionModel  default type to reqwest::Client ([#1013](https://github.com/wey-gu/rig/pull/1013)) (by @cody-why) - #1013
- *(deps)* upgrade rmcp dependency ([#1008](https://github.com/wey-gu/rig/pull/1008)) (by @adrianncovaci)
- release ([#1007](https://github.com/wey-gu/rig/pull/1007)) (by @github-actions[bot]) - #1007
- release ([#938](https://github.com/wey-gu/rig/pull/938)) (by @github-actions[bot]) - #938
- Fix bedrock tool calls with zero arguments ([#989](https://github.com/wey-gu/rig/pull/989)) (by @scrogson) - #989
- *(deps)* bump surrealdb from 2.3.6 to 2.3.8 ([#968](https://github.com/wey-gu/rig/pull/968)) (by @dependabot[bot])
- *(deps)* bump mongodb from 3.2.3 to 3.2.5 ([#957](https://github.com/wey-gu/rig/pull/957)) (by @dependabot[bot])
- Dependent packages no longer force unnecessary features on rig-core ([#964](https://github.com/wey-gu/rig/pull/964)) (by @nielsgreen) - #964
- InvalidCodeWithMessage error enum variant ([#963](https://github.com/wey-gu/rig/pull/963)) (by @joshua-mo-143) - #963
- *(rig-1003)* update list of production rig users ([#956](https://github.com/wey-gu/rig/pull/956)) (by @joshua-mo-143)
- make streaming prompt module pub ([#944](https://github.com/wey-gu/rig/pull/944)) (by @joshua-mo-143) - #944
- Update toolchain and flake ([#939](https://github.com/wey-gu/rig/pull/939)) (by @FayCarsons) - #939
- *(rig-993)* re-import all items from embeddings module in rig::embeddings ([#936](https://github.com/wey-gu/rig/pull/936)) (by @joshua-mo-143)
- release ([#877](https://github.com/wey-gu/rig/pull/877)) (by @github-actions[bot]) - #877
- *(rig-984)* rename rig fastembed example filenames ([#914](https://github.com/wey-gu/rig/pull/914)) (by @joshua-mo-143)
- Add `just` + WASM target and tooling to nix flake ([#901](https://github.com/wey-gu/rig/pull/901)) (by @FayCarsons) - #901
- add VT Code project to the README ([#893](https://github.com/wey-gu/rig/pull/893)) (by @vinhnx) - #893
- provider SDK has issue with DocumentBlock ([#892](https://github.com/wey-gu/rig/pull/892)) (by @mkranjac) - #892
- *(rig-975)* split streaming portion of PromptHook ([#889](https://github.com/wey-gu/rig/pull/889)) (by @joshua-mo-143)
- *(rig-975)* split streaming portion of PromptHook (by @joshua-mo-143)
- *(rig-959)* Documents in Huggingface are not converted properly ([#874](https://github.com/wey-gu/rig/pull/874)) (by @FayCarsons)
- release ([#822](https://github.com/wey-gu/rig/pull/822)) (by @github-actions[bot]) - #822
- *(rig-967)* add clause on AI assistance for coding in CONTRIBUTING.md ([#868](https://github.com/wey-gu/rig/pull/868)) (by @joshua-mo-143)
- *(rig-969)* update features on README ([#870](https://github.com/wey-gu/rig/pull/870)) (by @joshua-mo-143)
- *(rig-963)* fix feature regression in AWS bedrock ([#863](https://github.com/wey-gu/rig/pull/863)) (by @FayCarsons)
- fix typo in comment ([#866](https://github.com/wey-gu/rig/pull/866)) (by @letreturn) - #866
- *(rig-952)* use `rust-toolchain.toml` to pin Rust toolchain for dev ([#858](https://github.com/wey-gu/rig/pull/858)) (by @FayCarsons)
- Pin rust version in Nix flake ([#839](https://github.com/wey-gu/rig/pull/839)) (by @FayCarsons) - #839
- parse NDJSON correctly, fixes #825 ([#826](https://github.com/wey-gu/rig/pull/826)) (by @MariaGolubev) - #826
- make Reasoning non-exhaustive ([#830](https://github.com/wey-gu/rig/pull/830)) (by @hghalebi) - #830
- *(rig-949)* pin CI deps ([#834](https://github.com/wey-gu/rig/pull/834)) (by @FayCarsons)
- release ([#792](https://github.com/wey-gu/rig/pull/792)) (by @github-actions[bot]) - #792
- *(cohere)* use `reqwest-eventsource`, some code cleanup ([#815](https://github.com/wey-gu/rig/pull/815)) (by @joshua-mo-143)
- *(openAI, openrouter, deepseek, groq)* use `reqwest-eventsource` ([#814](https://github.com/wey-gu/rig/pull/814)) (by @joshua-mo-143)
- remove unnecessary clone ([#808](https://github.com/wey-gu/rig/pull/808)) (by @po1onius) - #808
- *(rig-917)* add code contribution guidelines ([#800](https://github.com/wey-gu/rig/pull/800)) (by @joshua-mo-143)
- *(rig-924)* update rmcp to 0.6 ([#785](https://github.com/wey-gu/rig/pull/785)) (by @joshua-mo-143)
- optional candidates token count ([#793](https://github.com/wey-gu/rig/pull/793)) (by @morenol) - #793
- allow prompt without preamble ([#791](https://github.com/wey-gu/rig/pull/791)) (by @morenol) - #791
- update twitter (by @0xMochan)
- release ([#729](https://github.com/wey-gu/rig/pull/729)) (by @github-actions[bot]) - #729
- Revert "feat(rig-915): function calling strict mode for deepseek ([#763](https://github.com/wey-gu/rig/pull/763))" ([#772](https://github.com/wey-gu/rig/pull/772)) (by @joshua-mo-143) - #772
- 修改文档错误 ([#771](https://github.com/wey-gu/rig/pull/771)) (by @cyberdoors) - #771
- *(rig-907)* use where clause for trait bounds ([#749](https://github.com/wey-gu/rig/pull/749)) (by @joshua-mo-143)
- *(rig-913)* add feature gated items to docs ([#764](https://github.com/wey-gu/rig/pull/764)) (by @joshua-mo-143)
- added AWS Bedrock client creation using from_env ([#710](https://github.com/wey-gu/rig/pull/710)) (by @mkranjac) - #710
- Remove duplicate methods in perplexity ([#725](https://github.com/wey-gu/rig/pull/725)) (by @cyberdoors) - #725
- release ([#723](https://github.com/wey-gu/rig/pull/723)) (by @github-actions[bot]) - #723
- release ([#719](https://github.com/wey-gu/rig/pull/719)) (by @github-actions[bot]) - #719
- release ([#676](https://github.com/wey-gu/rig/pull/676)) (by @github-actions[bot]) - #676
- *(rig-829)* upgrade schemars to v1.0.4 ([#715](https://github.com/wey-gu/rig/pull/715)) (by @joshua-mo-143)
- *(rig-883)* fully deprecate mcp feature flag ([#714](https://github.com/wey-gu/rig/pull/714)) (by @joshua-mo-143)
- *(gemini)* Refactor parts to Vec instead of OneOrMany in Gemini ([#691](https://github.com/wey-gu/rig/pull/691)) (by @quangIO)
- consistent visibility modifiers in openai ([#694](https://github.com/wey-gu/rig/pull/694)) (by @seafraf) - #694
- Update rmcp to version 0.5 ([#682](https://github.com/wey-gu/rig/pull/682)) (by @quangIO) - #682
- Fix SSE parsing in Gemini provider ([#683](https://github.com/wey-gu/rig/pull/683)) (by @quangIO) - #683
- *(rig-862)* remove sync bound from fn call() in tool trait ([#678](https://github.com/wey-gu/rig/pull/678)) (by @joshua-mo-143)
- 删除gemini providers中重复的方法 ([#675](https://github.com/wey-gu/rig/pull/675)) (by @cyberdoors) - #675
- release ([#673](https://github.com/wey-gu/rig/pull/673)) (by @github-actions[bot]) - #673
- remove unnecessary warning traces ([#672](https://github.com/wey-gu/rig/pull/672)) (by @joshua-mo-143) - #672
- *(rig-851)* update provider integrations list ([#651](https://github.com/wey-gu/rig/pull/651)) (by @joshua-mo-143)
- release ([#636](https://github.com/wey-gu/rig/pull/636)) (by @github-actions[bot]) - #636
- *(rig-861)* make Agent<M> non-exhaustive ([#670](https://github.com/wey-gu/rig/pull/670)) (by @joshua-mo-143)
- *(rig-860)* add .rustfmt.toml ([#668](https://github.com/wey-gu/rig/pull/668)) (by @joshua-mo-143)
- release ([#594](https://github.com/wey-gu/rig/pull/594)) (by @github-actions[bot]) - #594
- Add new claude models and default max tokens ([#634](https://github.com/wey-gu/rig/pull/634)) (by @Sytten) - #634
- *(rig-836)* deprecate mcp-core integration ([#631](https://github.com/wey-gu/rig/pull/631)) (by @joshua-mo-143)
- Refactor clients with builder pattern ([#615](https://github.com/wey-gu/rig/pull/615)) (by @Sytten) - #615
- change log level to debug for input/output ([#627](https://github.com/wey-gu/rig/pull/627)) (by @joshua-mo-143) - #627
- fix spelling issue  ([#607](https://github.com/wey-gu/rig/pull/607)) (by @rejected-l) - #607
- release ([#590](https://github.com/wey-gu/rig/pull/590)) (by @github-actions[bot]) - #590
- release ([#573](https://github.com/wey-gu/rig/pull/573)) (by @github-actions[bot]) - #573
- release ([#506](https://github.com/wey-gu/rig/pull/506)) (by @github-actions[bot]) - #506
- *(rig-802)* add justfile commands ([#559](https://github.com/wey-gu/rig/pull/559)) (by @joshua-mo-143)
- *(rig-803)* improve documentation for multi-turn ([#562](https://github.com/wey-gu/rig/pull/562)) (by @joshua-mo-143)
- update cargo.lock ([#548](https://github.com/wey-gu/rig/pull/548)) (by @joshua-mo-143) - #548
- *(rig-797)* remove feedback survey from README.md ([#547](https://github.com/wey-gu/rig/pull/547)) (by @joshua-mo-143)
- Migrate all crates to Rust 2024 ([#539](https://github.com/wey-gu/rig/pull/539)) (by @quangIO) - #539
- update deps ([#543](https://github.com/wey-gu/rig/pull/543)) (by @quangIO) - #543
- Declare shared dependencies in workspace ([#538](https://github.com/wey-gu/rig/pull/538)) (by @quangIO) - #538
- Bump lancedb ([#537](https://github.com/wey-gu/rig/pull/537)) (by @quangIO) - #537
- error fixes for clarity (by @eeemmmmmm) - #536
- Make clippy happy on all targets ([#542](https://github.com/wey-gu/rig/pull/542)) (by @quangIO) - #542
- *(rig-791)* documents not consistently added to DeepSeek prompts ([#528](https://github.com/wey-gu/rig/pull/528)) (by @joshua-mo-143)
- *(rig-788)* add policy for contributors owning work ([#521](https://github.com/wey-gu/rig/pull/521)) (by @joshua-mo-143)
- Fix `ToolResult` serialization in ollama provider ([#504](https://github.com/wey-gu/rig/pull/504)) (by @lnicola) - #504
- release ([#441](https://github.com/wey-gu/rig/pull/441)) (by @github-actions[bot]) - #441
- *(rig-778)* add "Who's using Rig?" partial list to main README.md ([#500](https://github.com/wey-gu/rig/pull/500)) (by @joshua-mo-143)
- Update lib.rs ([#497](https://github.com/wey-gu/rig/pull/497)) (by @MamunC0der) - #497
- Fix typo in AudioGenerationModel field name ([#487](https://github.com/wey-gu/rig/pull/487)) (by @davidjsonn) - #487
- Introduce Client Traits and Testing ([#440](https://github.com/wey-gu/rig/pull/440)) (by @yavens) - #440
- Only PDF docs are supported by their API ([#465](https://github.com/wey-gu/rig/pull/465)) (by @joshua-mo-143) - #465
- Add mistral provider ([#437](https://github.com/wey-gu/rig/pull/437)) (by @adrianncovaci) - #437
- Remove broken Cohere logo from README ([#456](https://github.com/wey-gu/rig/pull/456)) (by @dizer-ti) - #456
- `impl {Debug,Clone} for CompletionRequest` ([#457](https://github.com/wey-gu/rig/pull/457)) (by @Giovanni-Tably) - #457
- fix some typos in comment ([#445](https://github.com/wey-gu/rig/pull/445)) (by @eveneast) - #445
- *(rig-bedrock)* release v0.1.3 ([#435](https://github.com/wey-gu/rig/pull/435)) (by @github-actions[bot])
- release ([#402](https://github.com/wey-gu/rig/pull/402)) (by @github-actions[bot]) - #402
- Donot use async closure + Bump mcp-core ([#428](https://github.com/wey-gu/rig/pull/428)) (by @tarunKoyalwar) - #428
- Remove broken xAI reference link in embedding.rs ([#427](https://github.com/wey-gu/rig/pull/427)) (by @PixelPil0t1) - #427
- Style/trace gemini embedding ([#411](https://github.com/wey-gu/rig/pull/411)) (by @atellou) - #411
- Update agent_with_huggingface.rs ([#401](https://github.com/wey-gu/rig/pull/401)) (by @sky-coderay) - #401
- release ([#375](https://github.com/wey-gu/rig/pull/375)) (by @github-actions[bot]) - #375
- *(lancedb)* docs, make examples idempotent ([#387](https://github.com/wey-gu/rig/pull/387)) (by @joshua-mo-143)
- Updated broken link xaiAPI in `completion.rs` ([#384](https://github.com/wey-gu/rig/pull/384)) (by @cypherpepe) - #384
- Fix Clippy warnings for doc indentation and Error::other usage ([#364](https://github.com/wey-gu/rig/pull/364)) (by @strmfos) - #364
- release ([#356](https://github.com/wey-gu/rig/pull/356)) (by @github-actions[bot]) - #356
- New model provider: Anthropic Claude 3.7 Addition ([#341](https://github.com/wey-gu/rig/pull/341)) (by @affaan-m) - #341
- added mcp_tool + Example ([#335](https://github.com/wey-gu/rig/pull/335)) (by @stevohuncho) - #335
- release ([#334](https://github.com/wey-gu/rig/pull/334)) (by @github-actions[bot]) - #334
- Update openai.rs ([#340](https://github.com/wey-gu/rig/pull/340)) (by @affaan-m) - #340
- support svg ([#333](https://github.com/wey-gu/rig/pull/333)) (by @lispking) - #333
- release ([#309](https://github.com/wey-gu/rig/pull/309)) (by @github-actions[bot]) - #309
- rename DeepSeek_R1.pdf to deepseek_r1.pdf ([#316](https://github.com/wey-gu/rig/pull/316)) (by @0xMochan) - #316
- release ([#295](https://github.com/wey-gu/rig/pull/295)) (by @github-actions[bot]) - #295
- EchoChambers Example Integration ([#244](https://github.com/wey-gu/rig/pull/244)) (by @savageops) - #244
- deepseek message to remove dependencies with openai ([#283](https://github.com/wey-gu/rig/pull/283)) (by @carlos-verdes) - #283
- release ([#257](https://github.com/wey-gu/rig/pull/257)) (by @github-actions[bot]) - #257
- fix spelling errors in `Makefile` and `message.rs` ([#284](https://github.com/wey-gu/rig/pull/284)) (by @Pricstas) - #284
- Correct `tracing::debug` message. ([#275](https://github.com/wey-gu/rig/pull/275)) (by @M4n5ter) - #275
- agent recipes ([#215](https://github.com/wey-gu/rig/pull/215)) (by @joshua-mo-143) - #215
- Revert "feat: add Galadriel API integration ([#188](https://github.com/wey-gu/rig/pull/188))" ([#264](https://github.com/wey-gu/rig/pull/264)) (by @joshua-mo-143) - #264
- *(example)* fix grammar mistake ([#260](https://github.com/wey-gu/rig/pull/260)) (by @famouswizard)
- Fix typos  "substract" → "subtract" ([#256](https://github.com/wey-gu/rig/pull/256)) (by @Pronoss) - #256
- fix typos ([#242](https://github.com/wey-gu/rig/pull/242)) (by @bodhi-crypo) - #242
- add more provider notes ([#237](https://github.com/wey-gu/rig/pull/237)) (by @lispking) - #237
- release ([#246](https://github.com/wey-gu/rig/pull/246)) (by @github-actions[bot]) - #246
- release ([#203](https://github.com/wey-gu/rig/pull/203)) (by @github-actions[bot]) - #203
- *(rig-eternalai)* Add missing manifest fields + basic README ([#240](https://github.com/wey-gu/rig/pull/240)) (by @cvauclair)
- *(README)* Fix broken link to Conventional Commits doc ([#225](https://github.com/wey-gu/rig/pull/225)) (by @mdqst)
- Fix typos ([#233](https://github.com/wey-gu/rig/pull/233)) (by @Dimitrolito) - #233
- *(README)* broken link to Anthropic Claude image in README ([#219](https://github.com/wey-gu/rig/pull/219)) (by @wale-e)
- add buttons to readme & contributing update ([#204](https://github.com/wey-gu/rig/pull/204)) (by @mateobelanger) - #204
- *(rig-sqlite)* Add integration test ([#202](https://github.com/wey-gu/rig/pull/202)) (by @cvauclair)
- *(README)* add SQLite as a supported vector store ([#201](https://github.com/wey-gu/rig/pull/201)) (by @steebchen)
- release ([#180](https://github.com/wey-gu/rig/pull/180)) (by @github-actions[bot]) - #180
- *(rig-neo4j)* remove old tests ([#197](https://github.com/wey-gu/rig/pull/197)) (by @cvauclair)
- *(rig-neo4j)* Fix neo4j integration test ([#190](https://github.com/wey-gu/rig/pull/190)) (by @cvauclair)
- Add additional check for empty tool_calls ([#166](https://github.com/wey-gu/rig/pull/166)) (by @npuichigo) - #166
- Mock provider API in vector store integration tests ([#186](https://github.com/wey-gu/rig/pull/186)) (by @cvauclair) - #186
- fix comment ([#182](https://github.com/wey-gu/rig/pull/182)) (by @drewstone) - #182
- fix various typos (by @qwdsds) - #170
- fix typo (by @eltociear) - #174
- temporarily disable cargo publish dry-run check ([#162](https://github.com/wey-gu/rig/pull/162)) (by @cvauclair) - #162
- release ([#144](https://github.com/wey-gu/rig/pull/144)) (by @github-actions[bot]) - #144
- *(rig-mongodb)* fix mongodb flaky integration test ([#154](https://github.com/wey-gu/rig/pull/154)) (by @cygaar)
- *(rig-mongodb)* fix flaky test ([#153](https://github.com/wey-gu/rig/pull/153)) (by @cvauclair)
- fix examples link in README.md ([#150](https://github.com/wey-gu/rig/pull/150)) (by @s7v7nislands) - #150
- *(ollama-example)* implement example showcasing ollama ([#148](https://github.com/wey-gu/rig/pull/148)) (by @0xMochan)
- *(readme)* add reference to new docs website (by @mateobelanger)
- *(integration-tests)* LanceDB ([#136](https://github.com/wey-gu/rig/pull/136)) (by @marieaurore123)
- *(embeddings)* add embedding distance calculator module ([#142](https://github.com/wey-gu/rig/pull/142)) (by @marieaurore123)
- *(readme)* fix typo (by @mateobelanger)
- change wording in breaking changes warning (by @mateobelanger)
- Devops/ci add publish check ([#140](https://github.com/wey-gu/rig/pull/140)) (by @cvauclair) - #140
- release ([#135](https://github.com/wey-gu/rig/pull/135)) (by @github-actions[bot]) - #135
- *(integration test)* MongoDB ([#126](https://github.com/wey-gu/rig/pull/126)) (by @marieaurore123)
- *(integration test)* Neo4J ([#133](https://github.com/wey-gu/rig/pull/133)) (by @marieaurore123)
- *(integration test)* Qdrant ([#134](https://github.com/wey-gu/rig/pull/134)) (by @marieaurore123)
- Add `rig-derive` missing manifest fields ([#129](https://github.com/wey-gu/rig/pull/129)) (by @cvauclair) - #129
- *(readme)* add perplexity logo to integrations ([#112](https://github.com/wey-gu/rig/pull/112)) (by @mateobelanger)
- release ([#102](https://github.com/wey-gu/rig/pull/102)) (by @github-actions[bot]) - #102
- Inefficient context documents serialization ([#100](https://github.com/wey-gu/rig/pull/100)) (by @cvauclair) - #100
- Add rig-qdrant to workspace ([#101](https://github.com/wey-gu/rig/pull/101)) (by @cvauclair) - #101
- release ([#87](https://github.com/wey-gu/rig/pull/87)) (by @github-actions[bot]) - #87
- Merge pull request #79 from 0xPlaygrounds/docs/readme/new-logo (by @mateobelanger) - #79
- update deps version (by @mateobelanger) - #79
- add coloured logos for integrations (by @mateobelanger) - #79
- Merge remote-tracking branch 'origin/main' into docs/readme/new-logo (by @mateobelanger) - #79
- Merge pull request #56 from 0xPlaygrounds/feat/model-provider/16-add-gemini-completion-embedding-models (by @cvauclair) - #56
- *(gemini)* add utility config docstring (by @mateobelanger)
- *(gemini)* remove try_from and use serde deserialization (by @mateobelanger)
- Merge remote-tracking branch 'origin/main' into feat/model-provider/16-add-gemini-completion-embedding-models (by @mateobelanger) - #56
- Merge branch 'main' into feat/model-provider/16-add-gemini-completion-embedding-models (by @mateobelanger) - #56
- *(gemini)* separate gemini api types module, fix pr comments (by @mateobelanger)
- add debug trait to embedding struct (by @mateobelanger) - #56
- *(readme)* remove gemini mention in non-exhaustive list (by @mateobelanger)
- *(gemini)* add addtionnal types from the official documentation, add embeddings example
- *(provider-gemini)* test pre-commits
- *(provider-gemini)* Update readme entries, add gemini agent example
- Fix CD workflow missing dependency for rig-lancedv (by @cvauclair) - #51
- cargo fmt (by @marieaurore123) - #33
- lance db examples (by @marieaurore123) - #33
- Merge branch 'main' into feat(vector-store)/lancedb (by @marieaurore123) - #33
- Merge pull request #39 from 0xPlaygrounds/docs/pr-template (by @cvauclair) - #39
- *(pr)* fix pr template formatting (by @0xMochan)
- *(.github)* implements pull request templates (by @0xMochan)
- Merge pull request #35 from 0xPlaygrounds/docs/contributing (by @cvauclair) - #35
- *(contributing)* fixes from review (by @0xMochan)
- *(reamde)* add breaking changes disclaimer (by @0xMochan)
- *(contributing)* initial instructions (+ .gitignore update) (by @0xMochan)
- Merge pull request #27 from 0xPlaygrounds/feat/anthropic (by @cvauclair) - #27
- *(readme)* update badges link (by @mateobelanger)
- Fix docstrings (by @cvauclair) - #21
- Deprecate RagAgent and Model in favor of versatile Agent (by @cvauclair) - #21
- Make RagAgent VectorStoreIndex dynamic trait objects (by @cvauclair) - #21
- bump rig-mongodb version (by @cvauclair) - #11
- manual version bump (by @cvauclair) - #11
- release (by @github-actions[bot]) - #11
- Merge pull request #18 from 0xPlaygrounds/feat/perplexity-support (by @cvauclair) - #18
- adjust readme.md a bit more (by @0xMochan) - #17
- adjust readme.md (by @0xMochan) - #17
- Rig logo in readme (by @0xMochan) - #17
- Add logging of http errors (by @cvauclair) - #14
- *(ghaction)* add ci run on main pushes to trigger cache (by @mateobelanger)
- Fix CD workflow (by @cvauclair) - #9
- Add ability to call CD workflow manually (by @cvauclair) - #8
- fix CD workflow (by @cvauclair) - #8
- Update CD workflow (by @cvauclair) - #7
- Add CD workflow using release-plz (by @cvauclair) - #7
- fmt code (by @cvauclair) - #7
- Add basic CI workflow (by @cvauclair) - #7
- Improve rig-mongodb docstrings (by @cvauclair) - #6
- Bump verseion (by @cvauclair) - #6
- Improve docstrings across the board (by @cvauclair) - #6
- Update issue templates (by @cvauclair)
- Update issue templates (by @cvauclair)
- Rag module docstrings (by @cvauclair)
- Update README.md (by @Tachikoma000)
- bump verseion (by @cvauclair)
- Cargo fmt (by @cvauclair)
- update package readme (by @cvauclair)
- Merge branch 'main' of https://github.com/0xPlaygrounds/rig (by @cvauclair)
- More docstrings (by @cvauclair)
- Add more docstrings + minor refactoring (by @cvauclair)
- Re-add installation section (by @cvauclair)
- Update integrations README section (by @cvauclair)
- Clippy+fmt (by @cvauclair)
- Add note about integrations (by @cvauclair)
- README improvements (by @cvauclair)
- README update (by @cvauclair)
- Small refactor + version bump (by @cvauclair)
- bump version (by @cvauclair)
- Split Chat and Prompt traits + other stuff (by @cvauclair)
- Add mongodb sub-lib README (by @cvauclair)
- Flesh out mongodb cargo manifest (by @cvauclair)
- Fully remove anyhow from lib (by @cvauclair)
- Clippy+fmt (by @cvauclair)
- Generalize embeddings document (by @cvauclair)
- Add associated types to Tools (by @cvauclair)
- Add ExtractionError + separate prompt and chat functionality (by @cvauclair)
- OpenAI and Cohere clients (by @cvauclair)
- Add new errors to sub-crate (by @cvauclair)
- Add JsonError to VectorStoreError (by @cvauclair)
- Add EmbeddingError and VectorStoreError enums (by @cvauclair)
- Placeholder main READEME (by @cvauclair)
- Add CompletionError and PromptError (by @cvauclair)
- Move MongoDB in its own project (by @cvauclair)
- Add more Cargo metadata (by @cvauclair)
- README and package metadata update (by @cvauclair)
- Add license (by @cvauclair)
- Rename lib (by @cvauclair)
- Downgrade version for testing (by @cvauclair)
- Initial commit (by @cvauclair)

### Contributors

* @Kade-Powell
* @gold-silver-copper
* @arsenyinfo
* @github-actions[bot]
* @dependabot[bot]
* @abhicris
* @sergiomeneses
* @daric93
* @mateobelanger
* @fangkangmi
* @geraschenko
* @tomasz-feliksik
* @Shaurya-Sethi
* @fversaci
* @jdwil
* @ForeverAngry
* @BigtoC
* @FayCarsons
* @alwayys-afk
* @byQuexo
* @fu050409
* @aleksmeshr
* @indrazm
* @inqode-lars
* @majiayu000
* @BillionClaw
* @isSerge
* @wey-gu
* @DAMEK86
* @nate-trojian
* @qaqland
* @LHelge
* @kevinastock
* @sachin-punyani
* @marcbrooker
* @gabrielrondon
* @shlaikov
* @CMLKevin
* @demoray
* @dgrijalva
* @joaquinhuigomez
* @lgaches
* @marieaurore123
* @lmariscal
* @howenyap
* @joshua-mo-143
* @illegalcall
* @EinarasGar
* @Sarah-2003
* @McTr0
* @bnomei
* @waltronix
* @sonicxml
* @itsezc
* @s3anw3
* @adrianncovaci
* @iskng
* @Fromsko
* @howardpen9
* @atellou
* @snaumov
* @0xMochan
* @liamwh
* @Phoenix500526
* @CremboC
* @AjithPanneerselvam
* @renanvieira
* @Lochlanna
* @ckaznable
* @austinsimpsond41
* @micllam
* @dxvid-pts
* @darinkishore
* @Sagebati
* @milutinke
* @hghalebi
* @eibrahim95
* @FrancescoXX
* @KanonWY
* @redroy44
* @legietha
* @mylesberueda
* @acgxv
* @sopaco
* @chenyh1990
* @davidgraeff
* @martsokha
* @yexiyue
* @ThomasMarches
* @felix307253927
* @silentEAG
* @marc2332
* @bebiksior
* @Sytten
* @Tachikoma000
* @gj-zhang
* @simonjanssen
* @yazaldefilimone
* @vandenbogart
* @ronaldslc
* @Dev79844
* @lleukkun
* @heeen
* @TheoOiry
* @Knight-Ops
* @erlendp
* @edisontim
* @Numeration
* @kafonek
* @cody-why
* @jaybe1234
* @ribelo
* @windoze
* @scrogson
* @hadronzoo
* @nielsgreen
* @sinkingsugar
* @lefebvreb
* @pushkarm029
* @vinhnx
* @mkranjac
* @z0w0
* @letreturn
* @sternelee
* @danparizher
* @MariaGolubev
* @po1onius
* @arhangelskij
* @morenol
* @cyberdoors
* @Soroushsrd
* @dmackdev
* @quangIO
* @seafraf
* @yavens
* @appflowy
* @rajiknows
* @vrurg
* @sredman
* @rejected-l
* @ksaritek
* @snowmead
* @wonrax
* @rmsc
* @eeemmmmmm
* @JazzyMcJazz
* @lnicola
* @davisuga
* @MamunC0der
* @kosz78
* @davidjsonn
* @ProHaller
* @QwF8PatkuU
* @dizer-ti
* @Giovanni-Tably
* @eveneast
* @PixelPil0t1
* @tarunKoyalwar
* @jizizr
* @marcbowes
* @sky-coderay
* @cbrit
* @cypherpepe
* @strmfos
* @piotrostr
* @affaan-m
* @futreall
* @stevohuncho
* @zhongjingjogy
* @M4n5ter
* @qhliang
* @jerryshell
* @Verticaleap
* @VaibhavSatija10
* @lispking
* @GustavoWidman
* @danik-tro
* @451846939
* @s6nqou
* @threewebcode
* @obikata
* @savageops
* @carlos-verdes
* @Pricstas
* @jimmydottech
* @kristjanpeterson1
* @famouswizard
* @Pronoss
* @bodhi-crypo
* @dandagow3t
* @cvauclair
* @mdqst
* @Dimitrolito
* @AdrielC
* @wale-e
* @steebchen
* @cgfo
* @zhourunlai
* @npuichigo
* @drewstone
* @qwdsds
* @eltociear
* @cygaar
* @s7v7nislands
* @tarrencev
## [0.39.0](https://github.com/0xPlaygrounds/rig/compare/v0.38.2...v0.39.0) - 2026-06-19

### Added

- *(providers)* add VoyageAI rerank support ([#1917](https://github.com/0xPlaygrounds/rig/pull/1917)) (by @sergiomeneses)
- *(agent)* [**breaking**] sans-IO AgentRun state machine; both agent loops become thin drivers ([#1899](https://github.com/0xPlaygrounds/rig/pull/1899)) (by @gold-silver-copper)

### Fixed

- correct possessive pronoun typo in CONTRIBUTING.md ([#1865](https://github.com/0xPlaygrounds/rig/pull/1865)) (by @abhicris) - #1865
- *(tool)* [**breaking**] deterministic, duplicate-safe tool registration + cassette tests ([#1913](https://github.com/0xPlaygrounds/rig/pull/1913)) (by @gold-silver-copper)

### Other

- *(deps)* bump uuid from 1.23.1 to 1.23.3 ([#1907](https://github.com/0xPlaygrounds/rig/pull/1907)) (by @dependabot[bot])
- *(deps)* bump lopdf from 0.40.0 to 0.41.0 ([#1877](https://github.com/0xPlaygrounds/rig/pull/1877)) (by @dependabot[bot])
- *(deps)* bump http from 1.4.0 to 1.4.2 ([#1909](https://github.com/0xPlaygrounds/rig/pull/1909)) (by @dependabot[bot])
- *(deps)* bump futures-timer from 3.0.3 to 3.0.4 ([#1908](https://github.com/0xPlaygrounds/rig/pull/1908)) (by @dependabot[bot])
- *(examples)* add Gemini mid-stream disruption token-counting example ([#1918](https://github.com/0xPlaygrounds/rig/pull/1918)) (by @gold-silver-copper)
- *(tool)* back ToolSet with an IndexMap instead of HashMap + order Vec ([#1916](https://github.com/0xPlaygrounds/rig/pull/1916)) (by @gold-silver-copper)
- de-flake tracing span tests and deepseek permission_control race ([#1915](https://github.com/0xPlaygrounds/rig/pull/1915)) (by @gold-silver-copper) - #1915
- *(agent)* cassette-backed AgentRun coverage against real Gemini turns ([#1901](https://github.com/0xPlaygrounds/rig/pull/1901)) (by @gold-silver-copper)
- Fix streaming reasoning history order ([#1898](https://github.com/0xPlaygrounds/rig/pull/1898)) (by @gold-silver-copper) - #1898
- Fix context document ordering ([#1893](https://github.com/0xPlaygrounds/rig/pull/1893)) (by @gold-silver-copper) - #1893
- Point ecosystem link to awesome-rig ([#1895](https://github.com/0xPlaygrounds/rig/pull/1895)) (by @gold-silver-copper) - #1895
- Add Gemini Nano Banana image generation ([#1889](https://github.com/0xPlaygrounds/rig/pull/1889)) (by @gold-silver-copper) - #1889

### Contributors

* @dependabot[bot]
* @abhicris
* @gold-silver-copper
* @sergiomeneses
## [0.38.2](https://github.com/0xPlaygrounds/rig/compare/v0.38.1...v0.38.2) - 2026-06-09

### Fixed

- support Anthropic mid-conversation system role ([#1862](https://github.com/0xPlaygrounds/rig/pull/1862)) (by @fangkangmi) - #1862

### Other

- *(deps)* bump tonic-prost-build from 0.14.5 to 0.14.6 ([#1874](https://github.com/0xPlaygrounds/rig/pull/1874)) (by @dependabot[bot])
- *(deps)* bump convert_case from 0.10.0 to 0.11.0 ([#1875](https://github.com/0xPlaygrounds/rig/pull/1875)) (by @dependabot[bot])
- *(deps)* bump reqwest from 0.13.3 to 0.13.4 ([#1873](https://github.com/0xPlaygrounds/rig/pull/1873)) (by @dependabot[bot])
- *(deps)* bump reqwest-middleware from 0.5.1 to 0.5.2 ([#1876](https://github.com/0xPlaygrounds/rig/pull/1876)) (by @dependabot[bot])
- Remove rig-redis integration ([#1887](https://github.com/0xPlaygrounds/rig/pull/1887)) (by @gold-silver-copper) - #1887
- migrate Copilot tests to cassette replay ([#1882](https://github.com/0xPlaygrounds/rig/pull/1882)) (by @gold-silver-copper) - #1882
- Redis vector store integration ([#1509](https://github.com/0xPlaygrounds/rig/pull/1509)) (by @daric93) - #1509
- add Ryzome to README nav links ([#1879](https://github.com/0xPlaygrounds/rig/pull/1879)) (by @mateobelanger) - #1879
- [codex] support mistral.rs OpenAI-compatible reasoning ([#1864](https://github.com/0xPlaygrounds/rig/pull/1864)) (by @gold-silver-copper) - #1864
- convert DeepSeek live tests to cassettes ([#1870](https://github.com/0xPlaygrounds/rig/pull/1870)) (by @gold-silver-copper) - #1870
- [codex] add OpenRouter cassette-backed provider coverage ([#1869](https://github.com/0xPlaygrounds/rig/pull/1869)) (by @gold-silver-copper) - #1869
- convert xAI live tests to cassettes ([#1868](https://github.com/0xPlaygrounds/rig/pull/1868)) (by @gold-silver-copper) - #1868
- [codex] cover Anthropic streaming tool result batching ([#1863](https://github.com/0xPlaygrounds/rig/pull/1863)) (by @gold-silver-copper) - #1863

### Contributors

* @dependabot[bot]
* @gold-silver-copper
* @daric93
* @mateobelanger
* @fangkangmi
## [0.38.1](https://github.com/0xPlaygrounds/rig/compare/v0.37.1...v0.38.1) - 2026-06-02

### Other

- unify workspace crate versions ([#1853](https://github.com/0xPlaygrounds/rig/pull/1853)) (by @gold-silver-copper) - #1853

### Contributors

* @gold-silver-copper
## [0.37.1](https://github.com/0xPlaygrounds/rig/compare/rig-v0.37.0...rig-v0.37.1) - 2026-06-02

### Added

- *(rig-derive)* replace hand-rolled schema with schemars in #[rig_tool] ([#1576](https://github.com/0xPlaygrounds/rig/pull/1576)) (by @tomasz-feliksik)
- *(gemini)* expose streaming response metadata ([#1790](https://github.com/0xPlaygrounds/rig/pull/1790)) (by @mateobelanger)
- *(anthropic)* support document citations ([#1778](https://github.com/0xPlaygrounds/rig/pull/1778)) (by @temrjan)

### Fixed

- *(chatgpt)* Handle ChatGPT response.completed events without output field ([#1825](https://github.com/0xPlaygrounds/rig/pull/1825)) (by @geraschenko)
- *(rig-gemini-grpc)* populate FunctionDeclaration.parameters from ToolDefinition ([#1763](https://github.com/0xPlaygrounds/rig/pull/1763)) (by @abhicris)
- fix sqlite threshold and null tool call streaming ([#1786](https://github.com/0xPlaygrounds/rig/pull/1786)) (by @gold-silver-copper) - #1786

### Other

- *(deps)* bump mongodb from 3.6.0 to 3.7.0 ([#1848](https://github.com/0xPlaygrounds/rig/pull/1848)) (by @dependabot[bot])
- *(deps)* bump zerocopy from 0.8.48 to 0.8.50 ([#1847](https://github.com/0xPlaygrounds/rig/pull/1847)) (by @dependabot[bot])
- *(deps)* bump google-cloud-aiplatform-v1 from 1.10.0 to 1.11.0 ([#1846](https://github.com/0xPlaygrounds/rig/pull/1846)) (by @dependabot[bot])
- *(deps)* bump serde_json from 1.0.149 to 1.0.150 ([#1845](https://github.com/0xPlaygrounds/rig/pull/1845)) (by @dependabot[bot])
- *(deps)* bump tonic from 0.14.5 to 0.14.6 ([#1844](https://github.com/0xPlaygrounds/rig/pull/1844)) (by @dependabot[bot])
- Fix parsing of streamed function-call argument deltas ([#1828](https://github.com/0xPlaygrounds/rig/pull/1828)) (by @geraschenko) - #1828
- *(deps)* port dependency bumps and Rust 1.91 ([#1842](https://github.com/0xPlaygrounds/rig/pull/1842)) (by @gold-silver-copper)
- *(deps)* bump quick-xml from 0.39.4 to 0.40.1 ([#1818](https://github.com/0xPlaygrounds/rig/pull/1818)) (by @dependabot[bot])
- *(deps)* bump google-cloud-auth from 1.9.0 to 1.10.0 ([#1817](https://github.com/0xPlaygrounds/rig/pull/1817)) (by @dependabot[bot])
- Stabilize MongoDB vector search test ([#1841](https://github.com/0xPlaygrounds/rig/pull/1841)) (by @gold-silver-copper) - #1841
- fix VT Code line grammar in README ([#1824](https://github.com/0xPlaygrounds/rig/pull/1824)) (by @Shaurya-Sethi) - #1824
- [codex] Validate model tool calls ([#1823](https://github.com/0xPlaygrounds/rig/pull/1823)) (by @gold-silver-copper) - #1823
- [codex] apply Anthropic cache control to tools ([#1815](https://github.com/0xPlaygrounds/rig/pull/1815)) (by @gold-silver-copper) - #1815
- *(deps)* bump tokio-tungstenite from 0.23.1 to 0.28.0 ([#1784](https://github.com/0xPlaygrounds/rig/pull/1784)) (by @dependabot[bot])
- *(deps)* bump rmcp from 1.6.0 to 1.7.0 ([#1783](https://github.com/0xPlaygrounds/rig/pull/1783)) (by @dependabot[bot])
- *(deps)* bump tokio from 1.52.1 to 1.52.3 ([#1782](https://github.com/0xPlaygrounds/rig/pull/1782)) (by @dependabot[bot])
- Expose per-completion-call usage in agent responses ([#1787](https://github.com/0xPlaygrounds/rig/pull/1787)) (by @gold-silver-copper) - #1787
- *(gemini)* add streaming metadata cassettes ([#1777](https://github.com/0xPlaygrounds/rig/pull/1777)) (by @gold-silver-copper)
- Add replayable provider cassette tests ([#1769](https://github.com/0xPlaygrounds/rig/pull/1769)) (by @gold-silver-copper) - #1769

### Contributors

* @dependabot[bot]
* @geraschenko
* @tomasz-feliksik
* @gold-silver-copper
* @abhicris
* @Shaurya-Sethi
* @mateobelanger
* @temrjan
## [0.37.0](https://github.com/0xPlaygrounds/rig/compare/rig-v0.36.0...rig-v0.37.0) - 2026-05-13

### Added

- *(openrouter)* add transcription (STT) and audio generation (TTS) support ([#1757](https://github.com/0xPlaygrounds/rig/pull/1757)) (by @fversaci)
- *(rig-bedrock)* add structured output support via Converse API ([#1667](https://github.com/0xPlaygrounds/rig/pull/1667)) (by @jdwil)
- *(memory)* Rig-managed conversation memory + rig-memory companion crate ([#1702](https://github.com/0xPlaygrounds/rig/pull/1702)) (by @ForeverAngry)
- add copilot model listing ([#1700](https://github.com/0xPlaygrounds/rig/pull/1700)) (by @BigtoC) - #1700

### Fixed

- *(gemini)* Token usage correctness for posthog llm analytics ([#1761](https://github.com/0xPlaygrounds/rig/pull/1761)) (by @mateobelanger)
- *(core)* [**breaking**] make Chat append messages to caller history ([#1733](https://github.com/0xPlaygrounds/rig/pull/1733)) (by @gold-silver-copper)

### Other

- Clean up root facade features and integration docs ([#1764](https://github.com/0xPlaygrounds/rig/pull/1764)) (by @gold-silver-copper) - #1764
- fix "a ancient" grammar in glarb-glarb sample text ([#1755](https://github.com/0xPlaygrounds/rig/pull/1755)) (by @abhicris) - #1755
- *(deps)* bump lopdf from 0.36.0 to 0.40.0 ([#1754](https://github.com/0xPlaygrounds/rig/pull/1754)) (by @dependabot[bot])
- *(deps)* bump quick-xml from 0.39.2 to 0.39.4 ([#1752](https://github.com/0xPlaygrounds/rig/pull/1752)) (by @dependabot[bot])
- *(deps)* bump tonic-build from 0.14.5 to 0.14.6 ([#1751](https://github.com/0xPlaygrounds/rig/pull/1751)) (by @dependabot[bot])
- Move reusable test doubles into rig_core::test_utils ([#1745](https://github.com/0xPlaygrounds/rig/pull/1745)) (by @gold-silver-copper) - #1745
- workspace and docs cleanup ([#1742](https://github.com/0xPlaygrounds/rig/pull/1742)) (by @gold-silver-copper) - #1742
- openrouter vars ([#1741](https://github.com/0xPlaygrounds/rig/pull/1741)) (by @gold-silver-copper) - #1741
- Add provider file ID support for document inputs ([#1740](https://github.com/0xPlaygrounds/rig/pull/1740)) (by @gold-silver-copper) - #1740
- add smoke test for completion across all Copilot models ([#1730](https://github.com/0xPlaygrounds/rig/pull/1730)) (by @BigtoC) - #1730
- bump dependencies ([#1728](https://github.com/0xPlaygrounds/rig/pull/1728)) (by @gold-silver-copper) - #1728
- remove needless files ([#1715](https://github.com/0xPlaygrounds/rig/pull/1715)) (by @gold-silver-copper) - #1715
- AGENTS.MD, CONTRIBUTING.MD, and docs ([#1714](https://github.com/0xPlaygrounds/rig/pull/1714)) (by @gold-silver-copper) - #1714
- Add Bedrock integration tests ([#1707](https://github.com/0xPlaygrounds/rig/pull/1707)) (by @gold-silver-copper) - #1707

### Contributors

* @gold-silver-copper
* @fversaci
* @mateobelanger
* @abhicris
* @jdwil
* @dependabot[bot]
* @ForeverAngry
* @BigtoC
