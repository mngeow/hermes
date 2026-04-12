# End-to-End Examples

## Contents

- Chat agent with filesystem tools and runtime-loaded skills
- Approval-gated file edit agent with checkpointed resume
- A2UI-enabled coding agent over SSE

Assume Eino `v0.8+` for middleware examples. Check `go.mod` first and adapt field names if the pinned version differs.

## Chat Agent with Tools and Skills

Use this as the baseline for a production-style coding agent that can read files, execute shell commands, and load skills on demand.

```go
package main

import (
    "context"
    "log"
    "os"

    "github.com/cloudwego/eino-ext/adk/backend/local"
    "github.com/cloudwego/eino-ext/components/model/openai"

    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/components/model"
    fsmw "github.com/cloudwego/eino/adk/middlewares/filesystem"
    patchtoolcalls "github.com/cloudwego/eino/adk/middlewares/patchtoolcalls"
    skillmw "github.com/cloudwego/eino/adk/middlewares/skill"
    "github.com/cloudwego/eino/adk/middlewares/summarization"
)

func newChatModel(ctx context.Context) model.ToolCallingChatModel {
    cm, err := openai.NewChatModel(ctx, &openai.ChatModelConfig{
        APIKey: os.Getenv("OPENAI_API_KEY"),
        Model:  os.Getenv("OPENAI_MODEL"),
    })
    if err != nil {
        log.Fatal(err)
    }
    return cm
}

func newCodingAgent(ctx context.Context, skillsDir string) adk.Agent {
    cm := newChatModel(ctx)

    backend, err := local.NewBackend(ctx, &local.Config{})
    if err != nil {
        log.Fatal(err)
    }

    patchMW, err := patchtoolcalls.New(ctx, nil)
    if err != nil {
        log.Fatal(err)
    }

    fsMW, err := fsmw.New(ctx, &fsmw.MiddlewareConfig{
        Backend: backend,
        Shell:   backend,
    })
    if err != nil {
        log.Fatal(err)
    }

    skillBackend, err := skillmw.NewBackendFromFilesystem(ctx, &skillmw.BackendFromFilesystemConfig{
        Backend: backend,
        BaseDir: skillsDir,
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

    summaryMW, err := summarization.New(ctx, &summarization.Config{
        Model: cm,
    })
    if err != nil {
        log.Fatal(err)
    }

    agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
        Name:        "coding-agent",
        Description: "A Go coding agent that can use filesystem tools and skills.",
        Instruction: "Help with Go and Eino development. Use the skill tool when a relevant skill exists.",
        Model:       cm,
        Handlers: []adk.ChatModelAgentMiddleware{
            patchMW,
            fsMW,
            skillMW,
            summaryMW,
        },
    })
    if err != nil {
        log.Fatal(err)
    }

    return agent
}

func main() {
    ctx := context.Background()

    agent := newCodingAgent(ctx, os.Getenv("EINO_EXT_SKILLS_DIR"))
    runner := adk.NewRunner(ctx, adk.RunnerConfig{
        Agent:           agent,
        EnableStreaming: true,
    })

    iter := runner.Query(ctx, "Use the eino skill if needed and explain how to add a custom tool.")
    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        if event.Err != nil {
            log.Fatal(event.Err)
        }
        if msg, err := event.Output.MessageOutput.GetMessage(); err == nil {
            log.Println(msg.Content)
        }
    }
}
```

Notes:
- Add `reduction.New(...)` when tool outputs are large enough to threaten context quality.
- Keep `patchtoolcalls` first in the middleware chain.
- Keep skills and filesystem separate: skills provide instructions, filesystem provides action.

## Approval-Gated File Edit Agent

Use this when a tool must pause for human approval before mutating files or making external side effects. This pattern is based on the documented `compose.StatefulInterrupt(...)` flow and works with `Runner.ResumeWithParams(...)`.

```go
package main

import (
    "context"
    "fmt"
    "log"
    "os"

    "github.com/cloudwego/eino-ext/components/model/openai"

    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/components/tool"
    "github.com/cloudwego/eino/components/tool/utils"
    "github.com/cloudwego/eino/compose"
    "github.com/cloudwego/eino/schema"
)

type ApprovalInfo struct {
    ToolName        string `json:"tool_name"`
    ArgumentsInJSON string `json:"arguments_in_json"`
    ToolCallID      string `json:"tool_call_id"`
}

type ApprovalResult struct {
    Approved         bool    `json:"approved"`
    DisapproveReason *string `json:"disapprove_reason,omitempty"`
}

type ApprovalTool struct {
    tool.InvokableTool
}

func (t *ApprovalTool) Info(ctx context.Context) (*schema.ToolInfo, error) {
    return t.InvokableTool.Info(ctx)
}

func (t *ApprovalTool) InvokableRun(ctx context.Context, argumentsInJSON string, opts ...tool.Option) (string, error) {
    toolInfo, err := t.Info(ctx)
    if err != nil {
        return "", err
    }

    wasInterrupted, _, storedArguments := compose.GetInterruptState[string](ctx)
    if !wasInterrupted {
        return "", compose.StatefulInterrupt(ctx, &ApprovalInfo{
            ToolName:        toolInfo.Name,
            ArgumentsInJSON: argumentsInJSON,
            ToolCallID:      compose.GetToolCallID(ctx),
        }, argumentsInJSON)
    }

    isResumeTarget, hasData, data := compose.GetResumeContext[*ApprovalResult](ctx)
    if !isResumeTarget {
        return "", compose.StatefulInterrupt(ctx, &ApprovalInfo{
            ToolName:        toolInfo.Name,
            ArgumentsInJSON: storedArguments,
            ToolCallID:      compose.GetToolCallID(ctx),
        }, storedArguments)
    }
    if !hasData {
        return "", fmt.Errorf("tool %q resumed with no approval payload", toolInfo.Name)
    }
    if data.Approved {
        return t.InvokableTool.InvokableRun(ctx, storedArguments, opts...)
    }
    if data.DisapproveReason != nil {
        return fmt.Sprintf("tool %q disapproved: %s", toolInfo.Name, *data.DisapproveReason), nil
    }
    return fmt.Sprintf("tool %q disapproved", toolInfo.Name), nil
}

type ApplyPatchInput struct {
    Path    string `json:"path"`
    Content string `json:"content"`
}

func newFileEditAgent(ctx context.Context, checkpointStore compose.CheckPointStore) (*adk.Runner, error) {
    cm, err := openai.NewChatModel(ctx, &openai.ChatModelConfig{
        APIKey: os.Getenv("OPENAI_API_KEY"),
        Model:  os.Getenv("OPENAI_MODEL"),
    })
    if err != nil {
        return nil, err
    }

    applyPatchTool, err := utils.InferTool(
        "apply_patch_to_file",
        "Overwrite a file with new content after approval.",
        func(ctx context.Context, input *ApplyPatchInput) (string, error) {
            if err := os.WriteFile(input.Path, []byte(input.Content), 0644); err != nil {
                return "", err
            }
            return "patch applied", nil
        },
    )
    if err != nil {
        return nil, err
    }

    agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
        Name:        "patch-agent",
        Description: "Edits files, but always asks for approval first.",
        Instruction: "Draft the file change, then call apply_patch_to_file when ready.",
        Model:       cm,
        ToolsConfig: adk.ToolsConfig{
            ToolsNodeConfig: compose.ToolsNodeConfig{
                Tools: []tool.BaseTool{&ApprovalTool{InvokableTool: applyPatchTool}},
            },
        },
    })
    if err != nil {
        return nil, err
    }

    runner := adk.NewRunner(ctx, adk.RunnerConfig{
        Agent:           agent,
        EnableStreaming: true,
        CheckPointStore: checkpointStore,
    })
    return runner, nil
}

func main() {
    ctx := context.Background()

    var checkpointStore compose.CheckPointStore

    // Replace the nil store with a real checkpoint store in production.
    runner, err := newFileEditAgent(ctx, checkpointStore)
    if err != nil {
        log.Fatal(err)
    }

    iter := runner.Query(ctx, "Rewrite ./tmp/demo.txt to say hello from Eino", adk.WithCheckPointID("patch-approval-1"))

    var interruptID string
    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        if event.Err != nil {
            log.Fatal(event.Err)
        }
        if event.Action != nil && event.Action.Interrupted != nil {
            interruptID = event.Action.Interrupted.InterruptContexts[0].ID
            log.Printf("approval requested: %+v", event.Action.Interrupted.Data)
            break
        }
    }

    approved := &ApprovalResult{Approved: true}
    resumeIter, err := runner.ResumeWithParams(ctx, "patch-approval-1", &adk.ResumeParams{
        Targets: map[string]any{interruptID: approved},
    })
    if err != nil {
        log.Fatal(err)
    }

    for {
        event, ok := resumeIter.Next()
        if !ok {
            break
        }
        if event.Err != nil {
            log.Fatal(event.Err)
        }
    }
}
```

Notes:
- This is the cleanest documented approval pattern for tools because the tool keeps its own interrupt state.
- Use a distributed checkpoint store when the interrupt and resume happen on different machines.
- The resume payload is routed by interrupt ID via `ResumeWithParams`.

## A2UI-Enabled Coding Agent over SSE

Use this when you want the agent to drive a browser UI with incremental assistant output and interrupt requests. This example reuses the `newCodingAgent(...)` helper from the first section.

```go
package main

import (
    "context"
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "os"

    "github.com/cloudwego/eino/adk"
)

type ChatRequest struct {
    SessionID string `json:"sessionId"`
    Message   string `json:"message"`
}

type A2UIMessage struct {
    BeginRendering   *BeginRenderingMsg   `json:"beginRendering,omitempty"`
    DataModelUpdate  *DataModelUpdateMsg  `json:"dataModelUpdate,omitempty"`
    InterruptRequest *InterruptRequestMsg `json:"interruptRequest,omitempty"`
}

type BeginRenderingMsg struct {
    SurfaceID string `json:"surfaceId"`
}

type DataModelUpdateMsg struct {
    SurfaceID string `json:"surfaceId"`
    Text      string `json:"text"`
}

type InterruptRequestMsg struct {
    InterruptID string `json:"interruptId"`
    Reason      string `json:"reason"`
}

func writeSSE(w http.ResponseWriter, msg A2UIMessage) error {
    b, err := json.Marshal(msg)
    if err != nil {
        return err
    }
    _, err = fmt.Fprintf(w, "data: %s\n\n", b)
    if err != nil {
        return err
    }
    if f, ok := w.(http.Flusher); ok {
        f.Flush()
    }
    return nil
}

func main() {
    ctx := context.Background()

    agent := newCodingAgent(ctx, os.Getenv("EINO_EXT_SKILLS_DIR"))
    runner := adk.NewRunner(ctx, adk.RunnerConfig{
        Agent:           agent,
        EnableStreaming: true,
    })

    http.HandleFunc("/chat", func(w http.ResponseWriter, r *http.Request) {
        w.Header().Set("Content-Type", "text/event-stream")

        var req ChatRequest
        if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
            http.Error(w, err.Error(), http.StatusBadRequest)
            return
        }

        if err := writeSSE(w, A2UIMessage{
            BeginRendering: &BeginRenderingMsg{SurfaceID: "assistant"},
        }); err != nil {
            return
        }

        iter := runner.Query(r.Context(), req.Message,
            adk.WithSessionValues(map[string]any{"SessionID": req.SessionID}),
        )

        for {
            event, ok := iter.Next()
            if !ok {
                return
            }
            if event.Err != nil {
                http.Error(w, event.Err.Error(), http.StatusInternalServerError)
                return
            }

            if event.Action != nil && event.Action.Interrupted != nil {
                interruptID := event.Action.Interrupted.InterruptContexts[0].ID
                _ = writeSSE(w, A2UIMessage{
                    InterruptRequest: &InterruptRequestMsg{
                        InterruptID: interruptID,
                        Reason:      "User approval required",
                    },
                })
                continue
            }

            if msg, err := event.Output.MessageOutput.GetMessage(); err == nil && msg.Content != "" {
                _ = writeSSE(w, A2UIMessage{
                    DataModelUpdate: &DataModelUpdateMsg{
                        SurfaceID: "assistant",
                        Text:      msg.Content,
                    },
                })
            }
        }
    })

    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

Notes:
- Keep A2UI as a business-layer protocol around `AgentEvent`, not as agent core logic.
- If your UI needs resumable approvals, add a second endpoint that calls `runner.ResumeWithParams(...)` with the interrupt ID and user decision.
- If the session must survive process restarts, persist messages and pair the runner with a distributed checkpoint store.
