# agently CLI

Command-line interface for agently Edge Runtime.

## Installation

### From Source

```bash
cd agently-edge
cargo build --release
```

The binary will be at `target/release/agently`.

### Add to PATH

```bash
# Option 1: Copy to system path
sudo cp target/release/agently /usr/local/bin/

# Option 2: Add to your shell config
echo 'export PATH="$PWD/target/release:$PATH"' >> ~/.bashrc
```

## Usage

### Run an Agent

```bash
agently run <config.yaml>
```

### Verbose Mode

```bash
agently run <config.yaml> --verbose
```

### Examples

```bash
# Run the hello-world example
agently run examples/hello-world/agent.yaml

# Run with verbose output
agently run examples/hello-world/agent.yaml -v

# View help
agently --help
agently run --help
```

## Configuration

The CLI accepts a YAML configuration file. See [examples/hello-world/agent.yaml](../examples/hello-world/agent.yaml) for an example.

### Minimal Config

```yaml
agent:
  id: my-agent
  name: My Agent
  version: 0.1.0
  description: "My custom agent"

runtime:
  model_path: ./models/model.gguf
  context_length: 4096

prompts:
  system: You are a helpful assistant.
  welcome: Hello! How can I help?
```

## Commands

| Command | Description |
|---------|-------------|
| `run`   | Run an agent with configuration |
| `help`  | Print help information |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Command-line argument error |

## Development

```bash
# Build
cargo build

# Run from source
cargo run -p agently -- run examples/hello-world/agent.yaml

# Run tests
cargo test -p agently

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

Apache 2.0
