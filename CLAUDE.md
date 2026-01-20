# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Raven is a Rust-based JavaScript runtime system that provides two distinct execution models:

1. **Workers Runtime**: Cloudflare Workers-style HTTP server with `export default { fetch() }` entry point
2. **Operator Runtime**: Custom execution environment for system operations (user/group/permission management)

Both runtimes are built on top of a shared core JavaScript runtime powered by the Boa JavaScript engine.

## Architecture

### Workspace Structure

This is a Cargo workspace with three crates:

- **`crates/common`**: Core library containing all runtime implementations and bindings
  - `runtime/`: Core JavaScript execution engine and binding system
  - `workers/`: HTTP server implementation with Workers-style API
  - `operator/`: System operator runtime with identity management bindings
  - `terminal/`: PTY and terminal handling (in development)
  - `ui/`: UI components (in development)

- **`crates/server`**: Server binary (placeholder, not yet implemented)
- **`crates/monitor`**: Monitor binary (placeholder, not yet implemented)

### Core Runtime Design

The runtime architecture uses a **binding registry pattern**:

1. **JsRuntime** (`runtime/core.rs`): Base JavaScript execution environment using Boa engine
2. **BindingRegistry** (`runtime/bindings/registry.rs`): Dynamic binding injection system
3. **Import parsing** (`runtime/import.rs`): Analyzes JS imports to selectively inject bindings

Key design principle: Bindings are only injected if explicitly imported in JavaScript code, reducing overhead.

### Workers Runtime

Located in `crates/common/src/workers/`:

- Implements Cloudflare Workers-style API
- Entry point: `export default { fetch(request) { ... } }`
- Provides HTTP request/response handling
- Includes KV store binding for key-value storage
- Uses thread-local storage for per-request binding isolation

### Operator Runtime

Located in `crates/common/src/operator/`:

- Provides system-level operations
- Bindings for user, group, permission, and sudo management
- Designed for system administration tasks via JavaScript

## Development Commands

### Building

```bash
# Build all crates
cargo build

# Build with release optimizations
cargo build --release

# Build with debug info for profiling
cargo build --profile=release-with-debug
```

### Running Examples

```bash
# Run Workers runtime example (HTTP server on port 8787)
cargo run --example worker

# Run Operator runtime example (identity management)
cargo run --example operator
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p common
```

### Release Profiles

- **`release`**: Maximum optimization (opt-level='z', LTO=fat, stripped symbols)
- **`release-with-debug`**: Release optimizations with debug symbols for profiling
- **`coverage`**: Optimized for code coverage analysis

## Key Implementation Details

### Binding System

Bindings use a trait-based system (`NativeBinding` in `runtime/bindings/mod.rs`):

```rust
pub trait NativeBinding: Send + Sync {
    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue;
}
```

Bindings are registered in the `BindingRegistry` and accessed via thread-local storage during JavaScript execution.

### Import-Based Injection

The runtime parses JavaScript imports to determine which bindings to inject:

```javascript
import { KV, UTILS } from 'raven:bindings';
```

Only `KV` and `UTILS` bindings will be injected for this script, improving performance.

### Thread-Local Context

The runtime uses `thread_local!` storage for the binding registry, allowing safe concurrent execution of multiple JavaScript contexts without global state conflicts.

## Dependencies

- **boa_engine**: JavaScript engine (ECMAScript implementation in Rust)
- **boa_gc**: Garbage collection for Boa
- Additional: serde, chrono, rand, sha2, base64 for utility functions
