# Chat Agents

## Start Here

Use `ChatModel` when the task is still a model call with explicit prompt assembly. Move up to `ChatModelAgent` when the agent needs a tool loop, multi-turn state, event streaming, or sub-agent collaboration.

## Pick the Runtime

- Use `ChatTemplate` plus `ChatModel` for direct prompt-to-response flows.
- Use `ChatModelAgent` when the model should decide which tool to call and when to stop.
- Use workflow agents for deterministic sequential, loop, or parallel multi-agent compositions.
- Use supervisor or plan-execute only when the coordination pattern is clearly needed.
- Use `DeepAgents` as the closest prebuilt Eino pattern to a coding assistant.

## ChatModel Rules

- Build messages as `[]*schema.Message`.
- Use `schema.SystemMessage`, `schema.UserMessage`, and `schema.MessagesPlaceholder` instead of string-concatenated prompts.
- Remember that `WithTools` returns a new bound model instance.
- Implement or preserve common options like temperature, max tokens, top-p, and stop when writing custom model wrappers.
- Assume streaming and non-streaming use cases both matter.

## Example: Prompt Plus ChatModel

```go
package main

import (
    "context"
    "fmt"

    "github.com/cloudwego/eino/components/model"
    "github.com/cloudwego/eino/components/prompt"
    "github.com/cloudwego/eino/schema"
)

func run(ctx context.Context, cm model.BaseChatModel) error {
    tpl := prompt.FromMessages(
        schema.FString,
        schema.SystemMessage("You are a Go reviewer."),
        schema.MessagesPlaceholder("history", true),
        schema.UserMessage("Review this code: {code}"),
    )

    msgs, err := tpl.Format(ctx, map[string]any{
        "history": []*schema.Message{schema.UserMessage("Focus on bugs.")},
        "code":    "func add(a, b int) int { return a + b }",
    })
    if err != nil {
        return err
    }

    reply, err := cm.Generate(ctx, msgs)
    if err != nil {
        return err
    }

    fmt.Println(reply.Content)
    return nil
}
```

## ChatModelAgent Rules

- Run agent execution through `adk.Runner`, not only `agent.Run()`, when you need callbacks, events, or interrupt/resume.
- Treat `AgentEvent` as the main stream for UI, logging, traces, and tool lifecycle reporting.
- Treat `SessionValues` as per-run shared state across collaborating agents.
- Escape `{` and `}` in instructions if the code uses the default `GenModelInput` path, because default rendering uses pyfmt-style placeholders.
- Use `AgentAsTool` for specialists that should return results to a parent agent.

## Example: ChatModelAgent Plus Runner

```go
package main

import (
    "context"
    "fmt"
    "log"
    "os"

    "github.com/cloudwego/eino-ext/components/model/openai"
    "github.com/cloudwego/eino/adk"
    "github.com/cloudwego/eino/components/model"
    "github.com/cloudwego/eino/components/tool"
    "github.com/cloudwego/eino/components/tool/utils"
    "github.com/cloudwego/eino/compose"
)

func newChatModel() model.ToolCallingChatModel {
    cm, err := openai.NewChatModel(context.Background(), &openai.ChatModelConfig{
        APIKey: os.Getenv("OPENAI_API_KEY"),
        Model:  os.Getenv("OPENAI_MODEL"),
    })
    if err != nil {
        log.Fatal(err)
    }
    return cm
}

type GetWeatherInput struct {
    City string `json:"city"`
}

func main() {
    ctx := context.Background()

    weatherTool, err := utils.InferTool(
        "get_weather",
        "Gets the current weather for a specific city.",
        func(ctx context.Context, input *GetWeatherInput) (string, error) {
            return fmt.Sprintf("the temperature in %s is 25C", input.City), nil
        },
    )
    if err != nil {
        log.Fatal(err)
    }

    agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
        Name:        "WeatherAgent",
        Description: "Answers weather questions with a tool.",
        Instruction: "Use the get_weather tool when the user asks about weather.",
        Model:       newChatModel(),
        ToolsConfig: adk.ToolsConfig{
            ToolsNodeConfig: compose.ToolsNodeConfig{
                Tools: []tool.BaseTool{weatherTool},
            },
        },
    })
    if err != nil {
        log.Fatal(err)
    }

    runner := adk.NewRunner(ctx, adk.RunnerConfig{
        Agent:           agent,
        EnableStreaming: true,
    })

    iter := runner.Query(ctx, "What is the weather in Beijing?")
    for {
        event, ok := iter.Next()
        if !ok {
            break
        }
        if event.Err != nil {
            log.Fatal(event.Err)
        }
        if msg, err := event.Output.MessageOutput.GetMessage(); err == nil {
            fmt.Println(msg.Content)
        }
    }
}
```

## DeepAgents Guidance

- Reach for `DeepAgents` when the task is explicitly a coding-agent problem with filesystem, shell, todo planning, and sub-agents.
- Expect higher latency and token cost than a plain `ChatModelAgent` because planning and delegation add model calls.
- Preserve the repo's existing tool and middleware choices instead of piling a second coding stack on top.

## Agentic APIs

- Use `AgenticModel`, `AgenticChatTemplate`, and `AgenticToolsNode` only if the repo is already on `v0.9` or explicitly wants beta agentic surfaces.
- Prefer agentic APIs when provider-native server tools, MCP-rich content blocks, or structured reasoning/action blocks matter.
- Keep the whole loop consistent: standard message flows use `schema.Message`; agentic flows use `schema.AgenticMessage`.

## Good Defaults

- Keep prompts and tools narrow before adding supervisors or plan-execute.
- Keep deterministic business workflows outside the free-form agent loop.
- Use `RunPath` and agent names to keep multi-agent traces understandable.
- Call `SetLanguage` only at initialization time because it is global ADK state.

## Example: Agent as Tool

```go
bookAgent := NewBookRecommendAgent()
bookTool := adk.NewAgentTool(ctx, bookAgent)

agent, err := adk.NewChatModelAgent(ctx, &adk.ChatModelAgentConfig{
    Name:        "Router",
    Description: "Routes to specialist agents.",
    Model:       cm,
    ToolsConfig: adk.ToolsConfig{
        ToolsNodeConfig: compose.ToolsNodeConfig{
            Tools: []tool.BaseTool{bookTool},
        },
    },
})
```

Use `AgentAsTool` when the caller should keep control and consume the specialist result as another tool response.
