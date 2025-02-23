mod handler;
mod store;

use crate::handler::{get_handler, put_handler, AppState};
use crate::store::KvStore;
use axum::{routing::get, Router};
use config::{Config, ConfigError, Environment, File};
use std::process;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::fmt;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn load_config() -> Result<Config, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("VALORKV").separator("_"))
        ;
    builder.build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt().with_max_level(tracing::Level::INFO).init();

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    let data_file_path: String = config
        .get_string("data_file_path")
        .unwrap_or("data.bin".to_string());
    let bind_address: String = config
        .get_string("bind_address")
        .unwrap_or("0.0.0.0:6380".to_string());

    let kv_store = Arc::new(KvStore::new());

    if let Err(e) = kv_store.load_from_file(&data_file_path) {
        eprintln!("Failed to load data from file: {}", e);
        process::exit(1);
    }

    let app_state = AppState {
        kv_store: kv_store.clone(),
    };

    let app = Router::new()
        .route("/{key}", get(get_handler).put(put_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    kv_store.save_to_file(&data_file_path)?;
    info!("Data saved to file.");

    Ok(())
}
