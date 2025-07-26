# High Level Postgresql Sync

A command-line application for PostgreSQL synchronization tasks.

## Features

- Command-line interface with multiple subcommands
- PostgreSQL database connectivity using sqlx
- Configuration management via TOML files
- Database connection testing
- Configuration display

## Installation

Build from source:

```bash
cargo build --release
```

## Configuration

The application uses a `settings.toml` file for configuration. The default configuration includes:

```toml
[remote_database]
host = "localhost"
user = "postgres"
port = 5432
pgpass_path = "~/.pgpass"
database = "postgres"
```

### Configuration Options

- `host`: PostgreSQL server hostname
- `user`: Database username
- `port`: Database port (default: 5432)
- `pgpass_path`: Path to PostgreSQL password file
- `database`: Database name to connect to

## Usage

### Test Database Connection

```bash
hlps test
```

### Show Current Configuration

```bash
hlps config
```

### Custom Configuration File

```bash
hlps --config /path/to/custom/settings.toml test
```

## Requirements

- Rust 1.70 or later
- PostgreSQL server (for testing connections)

## Dependencies

- sqlx: PostgreSQL database connectivity
- tokio: Async runtime
- clap: Command-line argument parsing
- serde: Serialization/deserialization
- toml: TOML file parsing