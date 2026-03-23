# Hello World Agent

A minimal hello world agent demonstrating the basic structure of agently Edge Runtime.

## Features

- YAML configuration loading
- Interactive conversation loop
- Multi-turn context management
- Mock LLM response (ready for real LLM integration)

## Quick Start

### Prerequisites

- Rust 1.75+
- (Optional) Real LLM model for production use

### Build and Run

```bash
# Clone the repository
cd agently-edge/examples/hello-world

# Build
cargo build

# Run
cargo run
```

### Example Session

```
╔══════════════════════════════════════════════════════════╗
║  Hello World Agent v0.1.0                                ║
║  A friendly hello world assistant                        ║
╚══════════════════════════════════════════════════════════╝

👋 Hello! I'm your Hello World Agent.
I'm running on agently Edge Runtime.
How can I help you today?

Type 'quit' or 'exit' to stop.

👤 You: Hello!
🤖 Agent: Mock response to: Hello!

👤 You: How are you?
🤖 Agent: Mock response to: How are you?

👤 You: quit
👋 Goodbye! Thanks for using Hello World Agent.
```

## Configuration

Edit `agent.yaml` to customize your agent:

```yaml
agent:
  id: hello-world
  name: Hello World Agent
  version: 0.1.0

runtime:
  context_length: 4096
  temperature: 0.7

prompts:
  system: You are a friendly assistant.
  welcome: Hello! How can I help?
```

## Project Structure

```
hello-world/
├── Cargo.toml          # Project configuration
├── src/
│   ├── lib.rs          # Library (config, conversation context)
│   └── main.rs         # Binary entry point
├── tests/
│   ├── config_test.rs  # Configuration loading tests
│   └── conversation_test.rs  # Conversation tests
├── agent.yaml          # Agent configuration
└── README.md           # This file
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test --test config_test
cargo test --test conversation_test
```

## Next Steps

1. Integrate real LLM (llama.cpp)
2. Add tool calling support
3. Deploy to device

## License

Apache 2.0
