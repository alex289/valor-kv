#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::{
        process::{Child, Command},
        thread,
        time::Duration,
    };
    use valor_kv_client::KvStoreClient;

    #[derive(Serialize, Deserialize, Debug)]
    enum Message {
        Get(String),
        Set(String, String),
        Response(Option<String>),
    }

    fn start_server() -> Child {
        let server = Command::new("cargo")
            .arg("run")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start server");

        // Give the server some time to start
        thread::sleep(Duration::from_secs(3));

        server
    }

    fn close_server(mut server: Child) {
        server.kill().expect("Failed to kill server");
    }

    #[test]
    fn test_set_and_get() -> Result<(), Box<dyn std::error::Error>> {
        let server = start_server();

        let client = KvStoreClient::new("127.0.0.1:6380".to_string());

        client.set("testkey".to_string(), "testvalue".to_string())?;

        let value = client.get("testkey".to_string())?;
        assert_eq!(value, Some("testvalue".to_string()));

        close_server(server);

        Ok(())
    }

    #[test]
    fn test_get_nonexistent_key() -> Result<(), Box<dyn std::error::Error>> {
        let server = start_server();

        let client = KvStoreClient::new("127.0.0.1:6380".to_string());

        let value = client.get("nonexistent".to_string())?;
        assert_eq!(value, None);

        close_server(server);

        Ok(())
    }
}
