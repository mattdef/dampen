# Dampen LSP Server

Language Server Protocol (LSP) implementation for the Dampen UI framework.

## Overview

The Dampen LSP server provides real-time XML validation, intelligent autocompletion, and contextual hover documentation for `.dampen` files. It is built on the `tower-lsp` framework and communicates via JSON-RPC over stdio.

## Features

- **Real-time Validation**: Syntax and semantic errors appear as you type
- **Intelligent Autocompletion**: Context-aware suggestions for widgets, attributes, and values
- **Hover Documentation**: Documentation tooltips for widgets and attributes
- **Error Diagnostics**: Red underlines with detailed error messages and suggestions

## Installation

### From Source

```bash
cargo build --release -p dampen-lsp
```

The binary will be available at `target/release/dampen-lsp`.

### Prerequisites

- Rust 1.85+ (MSRV)
- dampen-core crate (included in workspace)

## Editor Configuration

### VS Code

Add to `.vscode/settings.json`:

```json
{
  "dampen.lsp.enabled": true,
  "dampen.lsp.path": "/path/to/dampen-lsp"
}
```

### Zed

Add to `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "dampen-lsp": {
      "binary": {
        "path": "/path/to/dampen-lsp"
      }
    }
  }
}
```

### Neovim

Using nvim-lspconfig:

```lua
require('lspconfig').dampen.setup{
  cmd = {"/path/to/dampen-lsp"},
  filetypes = {"dampen"},
  root_dir = require('lspconfig').util.root_pattern(".git", "Cargo.toml"),
}
```

## Usage

The LSP server is started automatically by your editor. Manual start:

```bash
dampen-lsp
```

The server reads JSON-RPC messages from stdin and writes responses to stdout.

## Development

### Running Tests

```bash
# All tests
cargo test -p dampen-lsp

# Specific test
cargo test -p dampen-lsp test_completion

# With output
cargo test -p dampen-lsp -- --nocapture
```

### Linting and Formatting

```bash
# Run clippy
cargo clippy -p dampen-lsp -- -D warnings

# Format code
cargo fmt -p dampen-lsp

# Check formatting
cargo fmt -p dampen-lsp -- --check
```

### Build Release

```bash
cargo build --release -p dampen-lsp
```

## Project Structure

```
crates/dampen-lsp/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── server.rs         # LSP server orchestration (in main.rs)
│   ├── document.rs       # Document cache management
│   ├── analyzer.rs       # Semantic analysis
│   ├── capabilities.rs   # LSP capabilities
│   ├── converters.rs     # Type conversions
│   ├── schema_data.rs    # Widget documentation
│   └── handlers/         # LSP method handlers
│       ├── mod.rs
│       ├── text_document.rs
│       ├── diagnostics.rs
│       ├── completion.rs
│       └── hover.rs
└── tests/
    ├── integration_tests.rs
    └── fixtures/
```

## Architecture

The server uses an actor-style architecture:

- **LspServer**: Main orchestrator handling LSP lifecycle
- **DocumentCache**: LRU cache of open documents (50 max)
- **Analyzer**: Semantic analysis and position-based queries
- **Handlers**: LSP method implementations (textDocument/*)

## Performance

- Parse time: <50ms for 1000-line files
- Completion response: <100ms
- Hover response: <200ms
- Diagnostics publish: <500ms

## Logging

Enable structured logging:

```bash
RUST_LOG=info dampen-lsp    # Info level
RUST_LOG=debug dampen-lsp   # Debug level
RUST_LOG=trace dampen-lsp   # Trace level (verbose)
```

## License

See the workspace LICENSE file.
