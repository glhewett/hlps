use clap::Parser;
use config::Config;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "hlps")]
#[command(about = "High Level Postgresql Sync - A command line application for PostgreSQL synchronization")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "settings.toml")]
    config: PathBuf,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Deserialize)]
struct Settings {
    remote_database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    user: String,
    port: u16,
    pgpass_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.verbose {
        println!("Starting High Level Postgresql Sync (hlps)");
        println!("Using configuration file: {:?}", args.config);
    }
    
    // Load configuration
    let settings = load_config(&args.config)?;
    
    if args.verbose {
        println!("Configuration loaded successfully:");
        println!("  Database host: {}", settings.remote_database.host);
        println!("  Database user: {}", settings.remote_database.user);
        println!("  Database port: {}", settings.remote_database.port);
        println!("  PgPass path: {}", settings.remote_database.pgpass_path);
    }
    
    println!("High Level Postgresql Sync is ready!");
    println!("Application configured for database: {}@{}:{}", 
             settings.remote_database.user, 
             settings.remote_database.host, 
             settings.remote_database.port);
    
    Ok(())
}

fn load_config(config_path: &PathBuf) -> Result<Settings, Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()?;
    
    let settings: Settings = settings.try_deserialize()?;
    Ok(settings)
}
