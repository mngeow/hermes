# HITL, Session, and Memory

## Session Boundaries

- Treat persistent memory and conversation history as application-layer concerns.
- Use stable session IDs so the app can reload prior messages from storage.
- Treat `SessionValues` as concurrency-safe shared run state, not as your durable database.
- Keep storage pluggable so JSONL, SQL, Redis, or object-store backends can be swapped without rewriting agent logic.

## Example: Inject and Read SessionValues

```go
package main

import (
    "context"
    "fmt"
    "os"
    "path/filepath"

    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/components/tool"
    "github.com/cloudwego/eino/components/tool/utils"
    "github.com/cloudwego/eino/schema"
)

type ReadRepoFileInput struct {
    Path string `json:"path"`
}

func NewReadRepoFileTool() (tool.InvokableTool, error) {
    return utils.InferTool(
        "read_repo_file",
        "Read a file relative to RepoRoot stored in SessionValues.",
        func(ctx context.Context, input *ReadRepoFileInput) (string, error) {
            rootAny, ok := adk.GetSessionValue(ctx, "RepoRoot")
            if !ok {
                return "", fmt.Errorf("RepoRoot not found in SessionValues")
            }

            root := rootAny.(string)
            b, err := os.ReadFile(filepath.Join(root, input.Path))
            if err != nil {
                return "", err
            }
            return string(b), nil
        },
    )
}

func runWithSession(ctx context.Context, runner *adk.Runner) {
    iter := runner.Run(ctx, []adk.Message{schema.UserMessage("Read go.mod")},
        adk.WithSessionValues(map[string]any{
            "RepoRoot": "/absolute/path/to/repo",
            "Language": "go",
        }),
    )

    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        _ = event
    }
}
```

## Durable Conversation Pattern

1. Accept or resolve a session ID.
2. Load stored messages for that session.
3. Build the next agent input from the restored history plus the new user message.
4. Run the agent through `adk.Runner`.
5. Persist the new messages or summarized state after the run.

## Interrupt and Resume

- Use interrupt/resume for approvals, review-and-edit loops, and long-running tasks that must survive process boundaries.
- Use checkpoints whenever the user must approve a risky tool call or when execution may outlive the current request.
- Keep the graph or agent configuration stable across resume; resume restores runtime state, not a new architecture.
- Register custom persisted types with the serializer or persistence mechanism used by the project.

## Example: Checkpointed Approval Loop

```go
package main

import (
    "context"
    "log"

    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/compose"
)

func runWithApproval(ctx context.Context, agent adk.Agent, checkpointStore compose.CheckPointStore) {
    runner := adk.NewRunner(ctx, adk.RunnerConfig{
        Agent:           agent,
        EnableStreaming: true,
        CheckPointStore: checkpointStore,
    })

    iter := runner.Query(ctx, "Apply the patch after approval.", adk.WithCheckPointID("approval-1"))
    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        if event.Err != nil {
            log.Fatal(event.Err)
        }
        if event.Action != nil && event.Action.Interrupted != nil {
            log.Printf("waiting for approval: %+v", event.Action.Interrupted.Data)
            break
        }
    }

    resumeIter, err := runner.Resume(ctx, "approval-1")
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

Use a real `compose.CheckPointStore` implementation in production. Use `adk.WithToolOptions(...)` on `Resume(...)` when the interrupted tool expects edited or approved user input.

## HITL Approval Pattern

1. Detect a risky action such as file writes, shell execution, or external side effects.
2. Raise an interrupt before the side effect happens.
3. Persist checkpoint state and surface the interrupt to the caller or UI.
4. Let the user approve, reject, or edit the proposed action.
5. Resume with user data and continue execution.

## Collaboration Rules

- Use `AgentAsTool` when a parent agent should keep control while delegating a specialized task.
- Use transfer only when control should move permanently to another agent.
- Remember that agent-as-tool isolates message history but still shares `SessionValues`.
- Enable internal event emission only when the caller genuinely needs sub-agent internals.

## Good Defaults

- Run production HITL flows through `Runner`.
- Design interrupt payloads and resume payloads explicitly; do not rely on prompt-only approvals.
- Use targeted resume when nested tools or nested agents can interrupt independently.
- Keep summarized history or transcript storage if long sessions may need dropped details later.
