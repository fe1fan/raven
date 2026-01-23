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

- **`crates/server`**: Web server binary built with Leptos framework
  - Provides web UI, HTTP API endpoints, and WebSocket connections
  - Runs on port 3000 (default)

- **`crates/monitor`**: Monitor binary (minimal implementation)

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
- HTTP server runs on port 8787 by default

**Script Execution**: JavaScript is wrapped in an IIFE that captures `export default`, then the exported handler's `fetch()` method is called for each HTTP request.

### Operator Runtime

Located in `crates/common/src/operator/`:

- Provides system-level operations
- Bindings for user, group, permission, and sudo management
- Designed for system administration tasks via JavaScript

**Script Execution**: JavaScript is wrapped in an async IIFE to allow top-level `await`, enabling async operations for system tasks.

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

# Run server with web UI (port 3000)
cd crates/server && cargo run
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

Bindings use a trait-based system (`NativeBinding` in `runtime/bindings/registry.rs`):

```rust
pub trait NativeBinding: Send + Sync {
    fn name(&self) -> &str;
    fn methods(&self) -> Vec<BindingMethod>;
    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue;
}
```

Bindings are registered in the `BindingRegistry` and accessed via thread-local storage during JavaScript execution.

**Value Marshalling**: All values passed between JavaScript and Rust go through the `BindingValue` enum, which supports: Null, Bool, Int, Float, String, Bytes, Json, Array, Object, and Error types. Conversion functions `js_to_binding_value()` and `binding_value_to_js()` handle bidirectional type translation.

### Import-Based Injection

The runtime parses JavaScript imports to determine which bindings to inject:

```javascript
import { KV } from 'raven/kv';
import { UTILS } from 'raven/utils';
```

Only the requested bindings will be injected for this script, improving performance. The import statements are removed during script preprocessing since Boa doesn't natively support ES6 modules. Bindings are injected into the global scope.

### Thread-Local Context

The runtime uses `thread_local!` storage for the binding registry, allowing safe concurrent execution of multiple JavaScript contexts without global state conflicts.

## Dependencies

- **boa_engine**: JavaScript engine (ECMAScript implementation in Rust)
- **boa_gc**: Garbage collection for Boa
- Additional: serde, chrono, rand, sha2, base64 for utility functions

## Available Bindings

### Workers Bindings

**KV** (`raven/kv`): Key-value storage
- `get(key)` - Retrieve value
- `put(key, value, options)` - Store value with optional TTL (options: `{expirationTtl: seconds}`)
- `delete(key)` - Remove value
- `list(options)` - List keys (options: `{prefix: string, limit: number}`)

**UTILS** (`raven/utils`): Utility functions
- `reverse(str)` - String reversal
- `base64Encode(str)` - Base64 encoding
- `base64Decode(str)` - Base64 decoding
- `hash(data)` - SHA-256 hashing
- `sum(array)` - Array sum
- `average(array)` - Array average
- `timestamp()` - Current timestamp

### Operator Bindings

Located in `crates/common/src/operator/bindings/identity/`:
- **UserManager** (`raven/user`): User account management
- **GroupManager** (`raven/group`): Group management
- **PermissionManager** (`raven/permission`): File permissions and ACLs
- **SudoManager** (`raven/sudo`): Sudo rule management

## Important Notes

- **Boa Limitation**: The engine doesn't natively support ES6 modules, so import statements are parsed and removed, with bindings injected into the global scope.
- **Console API**: `console.log()` and `console.error()` are available for debugging (output prefixed with `[JS]` or `[JS ERROR]`).
- **Async Support**: Operator runtime supports top-level `await` via async IIFE wrapping; Workers runtime uses synchronous execution model.
