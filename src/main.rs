use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hlps")]
#[command(about = "High Level Postgresql Sync - A PostgreSQL synchronization tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long, default_value = "settings.toml")]
    config: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Test database connection
    Test,
    /// Show current configuration
    Config,
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    remote_database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct DatabaseConfig {
    host: String,
    user: String,
    port: u16,
    pgpass_path: PathBuf,
    database: Option<String>,
}

impl Settings {
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }
}

impl DatabaseConfig {
    fn connection_string(&self) -> Result<String> {
        let database = self.database.as_deref().unwrap_or("postgres");
        
        // For now, we'll use a placeholder password
        // In a real implementation, you'd read from pgpass file
        let connection_string = format!(
            "postgresql://{}:password@{}:{}/{}",
            self.user, self.host, self.port, database
        );
        
        Ok(connection_string)
    }
}

async fn test_connection(config: &DatabaseConfig) -> Result<()> {
    println!("Testing connection to {}:{}...", config.host, config.port);
    
    let connection_string = config.connection_string()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await?;
    
    println!("Connection successful! Test query returned: {}", row.0);
    pool.close().await;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let settings = Settings::load_from_file(&cli.config)?;
    
    match cli.command {
        Some(Commands::Test) => {
            test_connection(&settings.remote_database).await?;
        }
        Some(Commands::Config) => {
            println!("Current configuration:");
            println!("Host: {}", settings.remote_database.host);
            println!("User: {}", settings.remote_database.user);
            println!("Port: {}", settings.remote_database.port);
            println!("Pgpass path: {}", settings.remote_database.pgpass_path.display());
        }
        None => {
            println!("High Level Postgresql Sync v0.1.0");
            println!("Use --help for available commands");
        }
    }
    
    Ok(())
}