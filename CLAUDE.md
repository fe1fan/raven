# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Raven is a Rust-based JavaScript runtime system powered by the Boa engine. It supports two primary execution models:
1.  **Workers Runtime**: Cloudflare Workers-style HTTP server (`export default { fetch() }`).
2.  **Operator Runtime**: Custom environment for system operations (user/group management).
3.  **Management UI**: A web-based dashboard for monitoring and controlling the runtime.

## Architecture

### Workspace Structure
- **`crates/common`**: Core runtime logic.
    - `runtime/`: Core JS engine, binding registry, and import parsing.
    - `workers/`: HTTP server and Workers-compatible API/bindings.
    - `operator/`: System management bindings and runtime.
- **`crates/server`**: Management UI and API.
    - Built with **Leptos (SSR mode)** and **Axum**.
    - `src/pages/`: File-organized Leptos page components.
    - `src/api/`: Axum REST API endpoints.
    - `src/websocket/`: WebSocket handlers for terminal/monitoring.
- **`crates/monitor`**: System monitoring agent.

### Core Design Patterns
- **Binding Registry**: Bindings are defined via the `NativeBinding` trait and registered in a `BindingRegistry`.
- **Import-Based Injection**: JS `import` statements are parsed to determine which Rust bindings to inject into the global scope (overcoming Boa's lack of native ESM).
- **Thread-Local Storage**: `JsRuntime` uses thread-local storage for the registry to ensure isolation during concurrent execution.

## Development Commands

### Building
```bash
cargo build                  # Build all crates
cargo build -p server        # Build only the management server
cargo build --release        # Build with 'z' optimization (size)
```

### Running
```bash
# Run Examples (from root)
cargo run --example worker     # Run workers runtime example (port 8787)
cargo run --example operator   # Run operator runtime example

# Run Management Server
cd crates/server && cargo run  # Web UI on port 3000
```

### Testing
```bash
cargo test                   # Run all tests
cargo test -p common         # Run tests for common crate
cargo test path::to::module  # Run specific test module
```

### Linting
```bash
cargo clippy                 # Run clippy for all targets
cargo fmt                    # Format code
```

## Implementation Notes
- **Leptos Setup**: The server uses Tailwind CSS (via CDN in dev) and glassmorphism styles. Dark mode is supported via a `dark` class on the `<html>` element.
- **Value Marshalling**: Bidirectional JS/Rust communication uses the `BindingValue` enum.
- **JS Preprocessing**: All JS code is wrapped in an IIFE (async for Operator, sync for Workers) before execution.
