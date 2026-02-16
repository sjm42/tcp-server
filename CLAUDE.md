# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A generic async TCP server for testing, built with Tokio. It listens on a configurable address, accepts connections, and prints received data to stdout. Uses `tracing` for structured logging with configurable verbosity levels.

## Build Commands

- `cargo build` — debug build
- `cargo build --release` — optimized release build (LTO enabled)
- `cargo build --profile minsize` — minimal binary size (stripped, abort on panic)
- `cargo clippy` — lint
- `cargo test` — run tests (no tests currently exist)

## Running

```
cargo run -- [OPTIONS]
```

Options: `-l/--listen <addr>` (default: `localhost:4242`), `-v` (info), `-d` (debug), `-t` (trace)

## Architecture

- **`src/bin/tcp_server.rs`** — Binary entry point. Manually builds a Tokio runtime (not using `#[tokio::main]`), binds a TCP listener, and spawns a task per connection that reads data and writes it to stdout.
- **`src/lib.rs`** — Library root. Re-exports `tracing::*` and `config::*`.
- **`src/config.rs`** — CLI argument parsing via `clap` derive. `OptsCommon` holds flags and listen address, provides `get_loglevel()` and `start_pgm()` which initializes tracing and logs build metadata.
- **`build.rs`** — Uses `build-data` crate to embed git branch, commit, source timestamp, and rustc version as compile-time environment variables.

## Conventions

- Rust stable toolchain (pinned in `rust-toolchain.toml`)
- Files end with `// EOF` comment
- Crate uses `pub use` re-exports in `lib.rs` so binaries can `use tcp_server::*` for flat access
