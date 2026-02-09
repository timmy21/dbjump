# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`dbjump` is a CLI tool for managing database connections through short aliases. It's written in Rust and acts as a configuration manager that delegates to native database CLI tools (e.g., `clickhouse client`, `psql`).

## Build and Development Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run a specific test
cargo test test_name

# Run the binary directly (development)
cargo run -- <command>

# Install locally
cargo install --path .
```

## Architecture

### Module Structure

- **`cli/`** - Command-line interface using clap
  - `args.rs` - CLI argument definitions and subcommands
  - `completions.rs` - Shell completion generation

- **`config/`** - Configuration management
  - `parser.rs` - TOML parsing, defines `Config`, `DatabaseConfig`, and `DatabaseEngine` enum
  - `path.rs` - Config file path resolution (`~/.config/dbjump/config.toml`)
  - `validator.rs` - Configuration validation logic

- **`database/`** - Database connection handling
  - `types.rs` - `DatabaseConnector` trait defining the interface for database adapters
  - `executor.rs` - Command execution logic (uses `exec()` on Unix to replace process)
  - `clickhouse.rs` - ClickHouse connector implementation
  - `postgresql.rs` - PostgreSQL connector implementation
  - `mod.rs` - Exports connectors and provides `get_connector()` factory function

- **`error.rs`** - Custom error types using `thiserror`
- **`utils.rs`** - Utility functions
- **`lib.rs`** - Library entry point exposing public API
- **`main.rs`** - Binary entry point with command routing

### Key Design Patterns

**DatabaseConnector Trait**: The core abstraction for database support. Each database type implements:
- `build_command()` - Constructs the `Command` with appropriate flags
- `cli_tool_name()` - Returns the CLI tool name for availability checks
- `check_availability()` - Verifies the CLI tool exists in PATH
- `format_preview()` - Formats connection info for display (passwords hidden)

**Process Replacement on Unix**: On Unix systems, `executor.rs` uses `CommandExt::exec()` to replace the current process with the database CLI. This preserves full interactive functionality (history, readline, etc.). On non-Unix systems, it falls back to `spawn()`.

**Optional Connection Parameters**: All connection fields (`host`, `port`, `user`, `password`, etc.) are `Option<T>` types. When omitted, the underlying database CLI uses its defaults.

## Adding a New Database Engine

1. Add variant to `DatabaseEngine` enum in `config/parser.rs`
2. Create new connector module in `database/` (e.g., `mysql.rs`)
3. Implement the `DatabaseConnector` trait
4. Update `get_connector()` in `database/mod.rs` to handle the new engine
5. Add tests for the new connector

## Configuration File

Location: `~/.config/dbjump/config.toml` (overridable via `DBJUMP_CONFIG` env var)

The config uses TOML with a `[[database]]` array. All fields except `alias` and `engine` are optional.

## Security Considerations

- Config directory permissions are set to 700 (owner-only)
- Config file permissions are set to 600 (owner read/write only)
- PostgreSQL passwords use `PGPASSWORD` env var to avoid process list exposure
- ClickHouse passwords use `--password` flag (less secure but required by the tool)
