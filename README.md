# High Level Postgresql Sync (hlps)

A command line application for PostgreSQL synchronization.

## Overview

hlps is a Rust-based command line tool designed for high-level PostgreSQL database synchronization operations.

## Configuration

The application uses a `settings.toml` file to store configuration. The default configuration includes:

```toml
[remote_database]
host = "localhost"
user = "postgres"
port = 5432
pgpass_path = "~/.pgpass"
```

## Usage

```bash
# Run with default settings.toml
./hlps

# Run with custom configuration file
./hlps --config /path/to/config.toml

# Run with verbose output
./hlps --verbose

# Show help
./hlps --help
```

## Requirements

- Rust (latest stable version)
- PostgreSQL connection details configured in settings.toml

## Building

```bash
cargo build --release
```

## Dependencies

- sqlx with PostgreSQL support
- tokio for async runtime
- clap for command line parsing
- config for TOML configuration reading
- serde for serialization/deserialization