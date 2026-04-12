# Version Notes

## Recommended Mental Baseline

- Treat `v0.8` as the important middleware-era baseline for coding agents.
- Treat `v0.7` as the important interrupt/resume redesign line.
- Treat `v0.6` as the JSON Schema migration line.
- Treat `v0.9` agentic APIs as beta.

## Key Changes

- `v0.6`: OpenAPI-oriented tool schema helpers were removed in favor of JSON Schema. Prefer `schema.ToJSONSchema()` and inferred Go structs.
- `v0.7`: interrupt and resume were refactored. Prefer modern interrupt-state and targeted-resume patterns over older examples.
- `v0.8`: `ChatModelAgentMiddleware` became the main extension seam. Prefer it over older ad hoc wrapper patterns.
- `v0.9`: `AgenticModel`, `AgenticChatTemplate`, and `AgenticToolsNode` became available as beta APIs.

## Old APIs to Avoid in New Code

- Do not teach `StateGraph` or `StateChain`; they were removed.
- Do not teach `GetState`; it was removed.
- Do not teach OpenAPI 3 tool schema types for modern Eino code.

## Filesystem Middleware Breaks in v0.8

- `read_file` offsets changed from 0-based to 1-based.
- `grep` pattern matching changed from literal matching to regex semantics.
- `write_file` now overwrites existing files.
- returned file paths are not guaranteed to be absolute.
- `ShellBackend` was renamed to `Shell`.

## Provider and Integration Caveats

- Keep `eino` and `eino-ext` versions aligned.
- Pin exact versions in `go.mod`; Eino is still pre-`v1` and APIs move.
- Treat streaming tool-call detection as provider-specific and add a custom `StreamToolCallChecker` when needed.
- Treat multimodal content and reasoning fields as integration-dependent, not guaranteed across every provider.
- Treat callback exporters like APMPlus, CozeLoop, or Langfuse as backend-specific layers on top of the same callback model.

## Doc Drift to Expect

- Some older llms-style quickstart links such as `simple_llm_application` and `agent_llm_with_tools` now resolve to `404` and have been replaced by chapter-based quickstarts.
- The cookbook lives under `/docs/eino/cookbook/`, not `/docs/eino/eino-cookbook/`.
- When docs and code disagree, prefer the imports and examples in the repo version already pinned by the project.

## Good Defaults

- Check `go.mod` before copying examples from docs.
- Prefer current quickstart chapters, ADK docs, and `eino-examples` patterns over older blog-style snippets.
- Prefer JSON Schema, `ChatModelAgentMiddleware`, and modern interrupt/resume flows in any new code you write.
