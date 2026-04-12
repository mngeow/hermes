# Tools, Skills, and Graphs

## Custom Tools

- Give every tool a precise name, description, and parameter schema.
- Prefer `InferTool` or `InferEnhancedTool` when a typed Go struct already expresses the input cleanly.
- Prefer enhanced tools when the result may need rich content like files, images, or structured tool output.
- Use sequential tool execution when calls depend on order; otherwise allow parallel execution.
- Use `compose.GetToolCallID(ctx)` inside tool execution or callbacks when correlation matters.

## Example: Custom Tool with InferTool

```go
package main

import (
    "context"
    "os"

    "github.com/cloudwego/eino/components/tool"
    "github.com/cloudwego/eino/components/tool/utils"
)

type ReadFileInput struct {
    Path string `json:"path" jsonschema_description:"Absolute file path to read"`
}

type ReadFileOutput struct {
    Content string `json:"content"`
}

func NewReadFileTool() (tool.InvokableTool, error) {
    return utils.InferTool(
        "read_file_excerpt",
        "Read a source file from disk for code analysis.",
        func(ctx context.Context, input *ReadFileInput) (*ReadFileOutput, error) {
            b, err := os.ReadFile(input.Path)
            if err != nil {
                return nil, err
            }
            return &ReadFileOutput{Content: string(b)}, nil
        },
    )
}
```

## ToolsNode Rules

- Use `ToolsNode` when the model already decided which tool to call.
- Use `AgenticToolsNode` only in agentic-message loops.
- Configure unknown-tool handling explicitly in agent systems that may see stale or hallucinated tool names.
- Keep tool side effects explicit and testable.

## Graph as Tool

- Use `compose.Graph` for deterministic flows with named nodes, loops, state, or parallel steps.
- Use `compose.Workflow` when you need field-level mapping between structs without wrapper types everywhere.
- Wrap a deterministic graph as a tool when an agent should invoke it opportunistically.
- Avoid burying a full conversational agent inside a graph node unless the behavior is actually bounded and synchronous.

## Example: Deterministic Graph Wrapped as a Tool

```go
package main

import (
    "context"

    "github.com/cloudwego/eino/components/model"
    "github.com/cloudwego/eino/components/prompt"
    "github.com/cloudwego/eino/components/tool"
    "github.com/cloudwego/eino/components/tool/utils"
    "github.com/cloudwego/eino/compose"
    "github.com/cloudwego/eino/schema"
)

func NewExplainRepoTool(ctx context.Context, cm model.BaseChatModel) (tool.InvokableTool, error) {
    g := compose.NewGraph[map[string]any, *schema.Message]()
    tpl := prompt.FromMessages(
        schema.FString,
        schema.SystemMessage("You explain Go repositories."),
        schema.UserMessage("{query}"),
    )

    _ = g.AddChatTemplateNode("prompt", tpl)
    _ = g.AddChatModelNode("model", cm)
    _ = g.AddEdge(compose.START, "prompt")
    _ = g.AddEdge("prompt", "model")
    _ = g.AddEdge("model", compose.END)

    runnable, err := g.Compile(ctx)
    if err != nil {
        return nil, err
    }

    type ExplainInput struct {
        Query string `json:"query"`
    }

    return utils.InferTool(
        "explain_repo",
        "Run a deterministic repository explanation flow.",
        func(ctx context.Context, input *ExplainInput) (string, error) {
            msg, err := runnable.Invoke(ctx, map[string]any{"query": input.Query})
            if err != nil {
                return "", err
            }
            return msg.Content, nil
        },
    )
}
```

## Filesystem Middleware

- Use filesystem middleware for coding agents that need `ls`, `read_file`, `write_file`, `edit_file`, `glob`, `grep`, or shell execution.
- Treat filesystem access and shell execution as separate capabilities.
- Keep tool names stable if the agent prompt or middleware stack already assumes them.
- Pair filesystem middleware with `ToolReduction` when file or shell outputs can be large.

## Skill Middleware

- Treat a skill as a folder containing `SKILL.md` plus optional `scripts/`, `references/`, and `assets/`.
- Remember that skill middleware only loads `SKILL.md`; it does not automatically grant filesystem or shell access.
- Use `skill.NewBackendFromFilesystem(...)` for filesystem-backed skill discovery.
- Store skills under a single base directory with one subdirectory per skill.
- Use `AgentHub` and `ModelHub` when Eino skill frontmatter uses context modes like `fork` or `fork_with_context`.

## Minimal Skill Wiring

```go
skillBackend, err := skill.NewBackendFromFilesystem(ctx, &skill.BackendFromFilesystemConfig{
    Backend: backend,
    BaseDir: skillsDir,
})
if err != nil {
    return err
}

skillMW, err := skill.NewMiddleware(ctx, &skill.Config{
    Backend: skillBackend,
})
if err != nil {
    return err
}
```

## Example: Filesystem Plus Skill Middleware

```go
package main

import (
    "context"
    "log"

    "github.com/cloudwego/eino/adk"
    fsmw "github.com/cloudwego/eino/adk/middlewares/filesystem"
    skillmw "github.com/cloudwego/eino/adk/middlewares/skill"
    localbk "github.com/cloudwego/eino-ext/adk/backend/local"
)

func main() {
    ctx := context.Background()

    backend, err := localbk.NewBackend(ctx, &localbk.Config{})
    if err != nil {
        log.Fatal(err)
    }

    fs, err := fsmw.New(ctx, &fsmw.MiddlewareConfig{
        Backend: backend,
    })
    if err != nil {
        log.Fatal(err)
    }

    skillBackend, err := skillmw.NewBackendFromFilesystem(ctx, &skillmw.BackendFromFilesystemConfig{
        Backend: backend,
        BaseDir: "/absolute/path/to/skills",
    })
    if err != nil {
        log.Fatal(err)
    }

    skillMW, err := skillmw.NewMiddleware(ctx, &skillmw.Config{
        Backend: skillBackend,
    })
    if err != nil {
        log.Fatal(err)
    }

    agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
        Name:        "CodeAgent",
        Description: "Agent with file access and runtime-loadable skills.",
        Model:       chatModel,
        Handlers:    []adk.ChatModelAgentMiddleware{fs, skillMW},
    })
    if err != nil {
        log.Fatal(err)
    }

    _ = agent
}
```

## Recommended Coding-Agent Middleware Order

- Put `PatchToolCalls` first when history may contain interrupted or missing tool results.
- Add filesystem middleware before skills if skills are expected to read files or run scripts.
- Add skill middleware so the agent can load specialized instructions on demand.
- Add summarization for long conversations.
- Add tool reduction when tool outputs can exceed practical prompt size.
- Add plan-task only when the agent should explicitly manage task state.
- Add tool-search only when the tool catalog is large enough that full tool lists hurt prompt budget.

## Practical Naming Rules

- Name tools for searchability because `ToolSearch` matches tool names, not descriptions.
- Keep tool schemas small and concrete.
- Keep graphs and tools separate when they serve different responsibilities.
- Keep reusable knowledge in skills and executable behavior in tools.
