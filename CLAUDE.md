# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

Woka is a CLI tool for connecting to remote development servers, built with Rust using clap for command-line parsing.

### Core Structure

- **src/main.rs**: Entry point that delegates to `accept::parse_and_run()`
- **src/accept/**: Command-line interface and argument parsing
  - Uses clap derive macros for CLI structure
  - Main commands: `connect` (default) and `server` (management)
  - Common options handle config file location (`-C/--config`)

- **src/config/**: Configuration data structures
  - `CommonConfigs`: Runtime configuration after parsing
  - `Server` enum: Currently only supports SSH servers
  - `SSHServer`: Contains display name and list of hostnames to try

- **src/apps/**: Application logic (currently minimal)

### Configuration System

- Default config location: `~/.config/woka/woka.toml`
- TOML format with `[default]` and `[server.*]` sections
- SSH servers can specify multiple hostnames to try in order
- Config parsing happens in `accept/common/file_config.rs`

### Command Structure

The CLI supports:
1. `woka [connect options]` - Connect to default/specified server
2. `woka connect [options]` - Explicit connect command  
3. `woka server add|list` - Server management (not yet implemented)

Current implementation is work-in-progress with many `todo!()` placeholders.