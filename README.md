# ValorKV: A Fast and Simple Key-Value Store

![Crates.io Version](https://img.shields.io/crates/v/valor_kv?style=flat)
![Crates.io Total Downloads](https://img.shields.io/crates/d/valor_kv)

ValorKV is a lightweight key-value store built in Rust, designed for speed and simplicity. It uses a custom TCP protocol for communication and bincode for efficient serialization.

## Features

*   **Fast TCP Communication:** Uses a custom TCP protocol for low-latency communication.
*   **Efficient Serialization:** Employs bincode for fast and compact data serialization.
*   **In-Memory Storage:** Stores data in memory for quick access.
*   **Persistence:** Saves data to disk on shutdown and loads it on startup.
*   **Configurable:** Uses a `config.toml` file and environment variables for configuration.
*   **Client Library:** Provides a Rust client library for easy integration.

## Getting Started

### Prerequisites

*   Rust (latest stable version)
*   Cargo (Rust's package manager)

### Server Installation

Using Docker:

```bash
docker run -p 6380:6380 -d ghcr.io/alex289/valor_kv:latest

#or

docker run -p 6380:6380 -d alexdev28/valor_kv:latest
```

Using Cargo:

```bash
cargo install valor_kv
valor_kv
```

### Using the Client Library

1.  **Add the `valor_kv_client` dependency to your `Cargo.toml`:**

    ```toml
    [dependencies]
    valor_kv_client = "0.1.0"  # Or the latest version
    ```

2.  **Use the client library in your Rust code:**

    ```rust
    use valor_kv_client::KvStoreClient;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = KvStoreClient::new("127.0.0.1:6380".to_string()); // Replace with your server address

        client.set("mykey".to_string(), "myvalue".to_string())?;
        let value = client.get("mykey".to_string())?;

        println!("Value for mykey: {:?}", value);

        Ok(())
    }
    ```

### Configuration

ValorKV can be configured using a `config.toml` file and environment variables.

#### `config.toml`

```toml
data_file_path = "data.bin"  # Path to the data file
bind_address = "0.0.0.0:6380" # Address to bind to
```

## Environment Variables

- VALORKV_DATA_FILE_PATH: Overrides the data_file_path setting.
- VALORKV_BIND_ADDRESS: Overrides the bind_address setting.

## Examples
See the client/examples directory for example code.

## Contributing

We welcome contributions! If you find any issues or have ideas for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. Feel free to use, modify, and distribute it as needed.