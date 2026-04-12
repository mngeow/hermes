# Observability and A2UI

## Callback Layers

- Use compose callbacks for component, node, and graph execution.
- Use agent callbacks for agent-level execution, but remember they only work through `adk.Runner`.
- Use callbacks for logs, metrics, traces, timing, token accounting, and stream visibility.
- Keep observability side effects out of business logic.

## Callback Rules

- Register global handlers once at initialization time; global callback registration is not concurrency-safe.
- Use per-run callback options when instrumentation should vary by request.
- Do not mutate callback input or output payloads.
- Close copied streams in handlers so callback plumbing does not block stream shutdown.
- Treat handler ordering as undefined.

## Example: Agent Callback Handler

```go
package main

import (
    "context"
    "fmt"

    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/callbacks"
)

func runWithCallbacks(ctx context.Context, runner *adk.Runner, input []adk.Message) {
    handler := callbacks.NewHandlerBuilder().
        OnStartFn(func(ctx context.Context, info *callbacks.RunInfo, input callbacks.CallbackInput) context.Context {
            if info.Component == adk.ComponentOfAgent {
                fmt.Printf("agent %s started\n", info.Name)
            }
            return ctx
        }).
        OnEndFn(func(ctx context.Context, info *callbacks.RunInfo, output callbacks.CallbackOutput) context.Context {
            if info.Component == adk.ComponentOfAgent {
                agentOutput := adk.ConvAgentCallbackOutput(output)
                go func() {
                    for {
                        event, ok := agentOutput.Events.Next()
                        if !ok {
                            break
                        }
                        fmt.Printf("event from %s: %+v\n", event.AgentName, event.Action)
                    }
                }()
            }
            return ctx
        }).
        Build()

    iter := runner.Run(ctx, input, adk.WithCallbacks(handler))
    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        _ = event
    }
}
```

## AgentEvent Rules

- Treat `AgentEvent` as the source of truth for UI streaming and agent lifecycle visibility.
- Use `RunPath` to make multi-agent traces understandable.
- Surface tool start, tool result, interruption, and final answer events in any debugging or UI layer.

## Eino Dev Tooling

- Use `github.com/cloudwego/eino-ext/devops` when the project wants visual graph debugging or orchestration tooling.
- Call `devops.Init(ctx)` before `Compile()`.
- Keep the process alive long enough for the tooling to attach.
- Add explicit type metadata when interface-typed or `map[string]any` values would otherwise hide concrete types.

## A2UI Boundary

- Treat A2UI as a business-layer protocol built on top of Eino events.
- Do not model A2UI as a core Eino runtime primitive.
- Use SSE for the quickstart-style integration unless the repo already uses another transport.

## Example: SSE Bridge from AgentEvent to A2UI-Style Messages

```go
package main

import (
    "context"
    "encoding/json"
    "fmt"
    "net/http"

    "github.com/cloudwego/eino/adk"
)

type A2UIMessage struct {
    DataModelUpdate  *DataModelUpdateMsg  `json:"dataModelUpdate,omitempty"`
    InterruptRequest *InterruptRequestMsg `json:"interruptRequest,omitempty"`
}

type DataModelUpdateMsg struct {
    SurfaceID string `json:"surfaceId"`
    Text      string `json:"text"`
}

type InterruptRequestMsg struct {
    InterruptID string `json:"interruptId"`
    Reason      string `json:"reason"`
}

func streamA2UI(ctx context.Context, w http.ResponseWriter, runner *adk.Runner, input string) error {
    w.Header().Set("Content-Type", "text/event-stream")
    flusher, ok := w.(http.Flusher)
    if !ok {
        return fmt.Errorf("response writer does not support flushing")
    }

    write := func(msg A2UIMessage) error {
        b, err := json.Marshal(msg)
        if err != nil {
            return err
        }
        if _, err := fmt.Fprintf(w, "data: %s\n\n", b); err != nil {
            return err
        }
        flusher.Flush()
        return nil
    }

    iter := runner.Query(ctx, input)
    for {
        event, ok := iter.Next()
        if !ok {
            return nil
        }
        if event.Err != nil {
            return event.Err
        }

        if event.Action != nil && event.Action.Interrupted != nil {
            err := write(A2UIMessage{
                InterruptRequest: &InterruptRequestMsg{
                    InterruptID: "approval-1",
                    Reason:      "User approval required",
                },
            })
            if err != nil {
                return err
            }
            continue
        }

        if msg, err := event.Output.MessageOutput.GetMessage(); err == nil {
            err = write(A2UIMessage{
                DataModelUpdate: &DataModelUpdateMsg{
                    SurfaceID: "assistant",
                    Text:      msg.Content,
                },
            })
            if err != nil {
                return err
            }
        }
    }
}
```

Replace the message structs with your app's actual A2UI v0.8 subset types. The important pattern is `Runner` event stream in, incremental SSE envelopes out.

## A2UI Message Model

- Expect one envelope message per SSE line.
- Common message types are `BeginRendering`, `SurfaceUpdate`, `DataModelUpdate`, `DeleteSurface`, and `InterruptRequest`.
- Quickstart examples implement a small subset of components such as `Text`, `Column`, `Card`, and `Row`.

## Event-to-UI Mapping

- Map assistant text streams to incremental `DataModelUpdate` messages.
- Map tool execution to progress or surface updates.
- Map approval pauses to `InterruptRequest` so the UI can collect user input.
- Keep the UI protocol stable even if the agent internals evolve.

## Good Defaults

- Add callbacks early in production code instead of bolting them on after bugs appear.
- Use backend-specific exporters from `eino-ext` only after the core callback design is correct.
- Keep SSE payloads small and incremental.
- Prefer stable node names and agent names so traces and UI updates remain readable.
