use paho_mqtt as mqtt;

pub async fn mqtt_client(max_retries: usize) -> mqtt::Result<mqtt::AsyncClient> {
    let client = mqtt::AsyncClient::new("mosquitto:1883").unwrap();

    for _ in 0..max_retries {
        if let Ok(_) = client.connect(mqtt::ConnectOptions::new()).await {
            return Ok(client);
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    client.connect(mqtt::ConnectOptions::new()).await.map(|_| client)
}
