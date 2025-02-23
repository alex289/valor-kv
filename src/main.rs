mod handler;
mod store;

use std::{net::TcpListener, process, sync::Arc};

use config::{Config, ConfigError, Environment, File};
use store::KvStore;
use tracing::{error, info, warn};
use tracing_subscriber::fmt;

fn load_config() -> Result<Config, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("VALORKV").separator("_"));
    builder.build()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt().with_max_level(tracing::Level::INFO).init();

    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    let data_file_path: String = config
        .get_string("data_file_path")
        .unwrap_or("data.bin".to_string());
    let bind_address: String = config
        .get_string("bind_address")
        .unwrap_or("0.0.0.0:6380".to_string());

    let listener = TcpListener::bind(&bind_address)?;
    info!("Listening on {}", bind_address);

    let kv_store = Arc::new(KvStore::new());

    if let Err(e) = kv_store.load_from_file(&data_file_path) {
        warn!("Failed to load data from file: {}", e);
    }

    let kv_store_clone = kv_store.clone();
    let data_file_path_clone = data_file_path.clone();
    ctrlc::set_handler(move || {
        if let Err(e) = kv_store_clone.save_to_file(&data_file_path_clone) {
            error!("Failed to save data to file: {}", e);
        } else {
            info!("Data saved to file.");
        }
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let kv_store_clone = kv_store.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handler::handle_client(stream, kv_store_clone) {
                        error!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }

    Ok(())
}
