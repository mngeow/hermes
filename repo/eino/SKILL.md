---
name: eino
description: Eino-native guidance for building Go LLM applications and agent systems with CloudWeGo Eino, Eino ADK, and eino-ext. Use when Codex is writing or reviewing Go code that uses `github.com/cloudwego/eino` or `eino-ext` for ChatModel or ChatModelAgent implementations, Graph or Workflow orchestration, custom tools, skill middleware, DeepAgents-style coding agents, human-in-the-loop interrupt/resume flows, session or memory persistence, callback or trace observability, or A2UI event-to-UI streaming.
---

# Eino

## Overview

Use Eino's typed components and orchestration model instead of treating the project like a generic LLM wrapper. Prefer `adk.ChatModelAgent` plus middleware for open-ended agents, and prefer `compose.Graph` or `compose.Workflow` for deterministic subflows that can be exposed as tools.

## Workflow

1. Inspect `go.mod` and imports before editing.
- `github.com/cloudwego/eino` usually means core components, compose, flow, schema, or callbacks.
- `github.com/cloudwego/eino/adk` means agent runtime work.
- `github.com/cloudwego/eino-ext` means providers, filesystem backends, callbacks exporters, devtools, or other integrations.
- If you see `agentic_*` surfaces, read `references/chat-agents.md` and `references/version-notes.md` first because those APIs are beta in `v0.9`.

2. Pick the narrowest Eino layer that fits.
- Use `ChatTemplate` plus `ChatModel` for single-turn or tightly controlled flows.
- Use `adk.NewChatModelAgent` for conversational, tool-using, multi-turn agents.
- Use `compose.Graph` or `compose.Workflow` when the flow is deterministic, parallelizable, or easier to debug as named nodes.
- Wrap a deterministic graph as a tool instead of embedding complex agent logic inside a graph node.
- Use `DeepAgents` only when you want a coding-agent stack with filesystem, shell, planning, and sub-agents out of the box.

3. Load the reference file that matches the task.
- Chat agents, ADK, runner, DeepAgents, or agentic APIs: `references/chat-agents.md`
- Custom tools, graph-as-tool, filesystem middleware, or skill loading: `references/tools-skills-and-graphs.md`
- Persistent conversations, session values, approval flows, or resume logic: `references/hitl-session-memory.md`
- Callbacks, traces, visual debugging, or streaming UI integration: `references/observability-and-a2ui.md`
- Version drift, stale docs, or provider-specific traps: `references/version-notes.md`
- Full wiring examples for common setups: `references/end-to-end-examples.md`
- Each reference file includes compact Go examples; start there before inventing your own shape.

## Decision Guide

- Need one prompt in and one answer out: use `ChatTemplate -> ChatModel`.
- Need a chat agent that decides which tool to call: use `ChatModelAgent`.
- Need a deterministic multi-step workflow: use `Graph` or `Workflow`, then expose it as a tool if an agent should invoke it.
- Need a specialist that returns control to a parent agent: use `AgentAsTool`.
- Need a production coding agent baseline: start from `DeepAgents` or a `ChatModelAgent` with filesystem, skill, patch-tool-calls, summarization, and tool-reduction middleware.
- Need provider-native server tools, MCP-rich content blocks, or deeper reasoning/action traces: consider `AgenticModel`, `AgenticChatTemplate`, and `AgenticToolsNode` only if the repo is already on `v0.9`.

## Core Defaults

- Prefer typed structs over `map[string]any` unless a boundary truly requires it.
- Prefer `schema.MessagesPlaceholder` for history injection.
- Prefer `InferTool` or `InferEnhancedTool` for custom tools backed by Go structs.
- Prefer JSON Schema-based tool definitions; do not add old OpenAPI-style schemas.
- Prefer `adk.Runner` whenever you need streaming events, callbacks, interrupt/resume, or resumable approvals.
- Prefer `AgentAsTool` over transfer when the caller should keep control.
- Prefer middleware for retries, history repair, summarization, and tool-output control instead of ad hoc prompt logic.
- Prefer callbacks for tracing, metrics, and logging instead of instrumenting business logic directly.
- Treat durable memory and session storage as application-layer work, not a built-in Eino subsystem.
- Treat A2UI as a business-layer protocol built from `AgentEvent` streams, not as a core Eino primitive.

## Minimal Stack

```go
agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
    Name:        "coding-agent",
    Description: "Eino coding assistant",
    Instruction: "Help with Go and Eino development tasks.",
    Model:       cm,
    Handlers: []adk.ChatModelAgentMiddleware{
        patchToolCallsMW,
        filesystemMW,
        skillMW,
        summarizationMW,
        toolReductionMW,
    },
})

runner := adk.NewRunner(ctx, adk.RunnerConfig{Agent: agent})
```

Add `planTask`, approval, or custom middleware only when the use case needs them.

## Reference Map

- `references/chat-agents.md`: `ChatModel`, `ChatTemplate`, `ChatModelAgent`, workflow agents, `DeepAgents`, and agentic surfaces.
- `references/tools-skills-and-graphs.md`: custom tools, `ToolsNode`, graph-as-tool, filesystem middleware, skill middleware, and coding-agent middleware stacks.
- `references/hitl-session-memory.md`: session values, durable conversation storage, interrupt/resume, checkpoints, and approval flows.
- `references/observability-and-a2ui.md`: compose callbacks, agent callbacks, Eino Dev tooling, `AgentEvent` streaming, and A2UI over SSE.
- `references/version-notes.md`: `v0.6` to `v0.9` changes, stale doc links, and provider-specific caveats.
- `references/end-to-end-examples.md`: full examples for a tool-and-skill coding agent, an approval-gated file edit agent, and an A2UI SSE server.
