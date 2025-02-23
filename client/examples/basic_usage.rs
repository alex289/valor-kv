use valor_kv_client::KvStoreClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your server address
    let client = KvStoreClient::new("127.0.0.1:6380".to_string());

    // Set a key-value pair
    client.set("mykey".to_string(), "myvalue".to_string())?;
    println!("Set mykey to myvalue");

    // Get the value for the key
    let value = client.get("mykey".to_string())?;
    println!("Value for mykey: {:?}", value.unwrap());

    Ok(())
}
