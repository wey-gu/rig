# Nowledge Mem compatibility branch

Based on upstream Rig v0.42.0. This branch carries only the compatibility work
still required by Nowledge Mem after upgrading from its v0.38.2 fork.

| Contract | Remaining patch |
| --- | --- |
| Headless ChatGPT OAuth cannot wait indefinitely | Injectable OAuth HTTP client; bounded defaults; token-safe deadline errors |
| OpenAI-compatible system prompts | Serialize system content as a plain string |
| Provider cache accounting | Preserve `prompt_tokens_details.cache_write_tokens` in normalized usage |
| Strict tool object schemas | Normalize snake-case additional properties and close every object schema |
| Responses gateway capabilities | Consume existing caller controls for instruction placement and omitted temperature/output limits, using the upstream instruction placement policy |
| Delta-less Responses output | Recover unseen text/refusal parts from done/completed frames without duplicating streamed parts |
| Mem Bazel source contract tests | Export the OAuth sources through a small Bazel package |

Upstream now covers noninteractive authentication, Anthropic string tool-input
coercion, reasoning aliases, id-less assistant input, completed tool identity
assembly, and ChatGPT empty terminal output. Their old patches are not replayed.

Rig 0.42 separates the `rig` facade, `rig-core`, and `rig-agent`. Consumers must
use the facade for agents, migrate hooks/dynamic tools, and preserve old durable
message formats at their own persistence boundary. App and Cloud must pin this
same commit. See nowledge-co/mem#3473 for consumer integration and validation.

The OpenAI-compatible replay cassettes deliberately update only system request
content from text arrays to the string form emitted by this compatibility
branch. Recorded responses remain unchanged; these request edits are not new
live-provider recordings.
