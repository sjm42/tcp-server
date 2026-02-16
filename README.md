# tcp-server

A lightweight async TCP server for testing and debugging network traffic. Accepts concurrent connections and dumps received data to stdout, optionally prefixed with connection identifiers for multiplexed output.

## Usage

```
cargo run -- [OPTIONS]
```

### Options

| Flag | Description |
|------|-------------|
| `-l, --listen <ADDR>` | Bind address (default: `localhost:4242`) |
| `-v, --verbose` | Enable info-level logging |
| `-d, --debug` | Enable debug-level logging (includes build metadata) |
| `-t, --trace` | Enable trace-level logging |

### Examples

Listen on the default port and print raw data:

```
cargo run
```

Listen on a custom address with connection-tagged output:

```
cargo run -- -v -l 0.0.0.0:9000
```

Test with netcat:

```
echo "hello" | nc localhost 4242
```

## Building

```
cargo build              # debug build
cargo build --release    # optimized (LTO, opt-level 3)
cargo build --profile minsize  # smallest binary (stripped, LTO, abort on panic)
```

## Internals

### Runtime Setup

The binary in `src/bin/tcp_server.rs` constructs a Tokio multi-threaded runtime manually rather than using the `#[tokio::main]` macro. This keeps `main()` synchronous — it parses CLI args and initializes tracing before handing off to the async `run_server()` via `runtime.block_on()`.

### Connection Handling

The server runs an accept loop that assigns each connection a monotonically increasing ID (`u64`). Each accepted connection is dispatched to its own Tokio task via `tokio::spawn`, allowing fully concurrent handling. The connection number and client address are moved into the spawned future.

Each connection task (`process_conn`) reads into a 64 KB stack-allocated buffer in a loop. When data arrives, it locks stdout, optionally writes a `[#N]` prefix (when log level is INFO or higher), writes the raw bytes, and flushes. The stdout lock is held for the minimum duration — just the write of one chunk — to avoid interleaving output from concurrent connections. When the read returns 0 bytes (EOF), the connection closes cleanly.

### Configuration & CLI

`src/config.rs` defines `OptsCommon` using clap's derive API. Log levels are mutually escalating flags: the highest specified flag wins (`-t` > `-d` > `-v`), defaulting to ERROR-only. The `start_pgm()` method initializes `tracing-subscriber` with the resolved level and logs version and build metadata at debug level.

### Build-Time Metadata

`build.rs` uses the `build-data` crate to capture git branch, commit hash, source timestamp, and rustc version as compile-time environment variables. These are embedded into the binary and printed on startup at debug verbosity. `no_debug_rebuilds()` prevents unnecessary recompilation when only debug profile settings change.

### Library Structure

`src/lib.rs` re-exports everything from `tracing` and `config` at the crate root, so the binary can simply `use tcp_server::*` to get flat access to all types, macros, and the `OptsCommon` struct.

## License

See [LICENSE](LICENSE).
