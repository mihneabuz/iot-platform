use std::env;

use influxdb as influx;
use paho_mqtt as mqtt;

pub fn get_mqtt_host() -> String {
    env::var("MQTT_HOST").unwrap_or("localhost".to_string())
}

pub fn get_influx_host() -> String {
    env::var("INFLUX_HOST").unwrap_or("localhost".to_string())
}

pub fn get_influx_auth() -> (String, String) {
    (
        env::var("INFLUX_USERNAME").unwrap_or("influxdb".to_string()),
        env::var("INFLUX_PASSWORD").unwrap_or("password".to_string()),
    )
}

pub async fn mqtt_client(max_retries: usize) -> mqtt::Result<mqtt::AsyncClient> {
    let client = mqtt::AsyncClient::new(get_mqtt_host() + ":1883").unwrap();

    for _ in 0..max_retries {
        if let Ok(_) = client.connect(mqtt::ConnectOptions::new()).await {
            return Ok(client);
        }

        log::warn!("could not connect to mqtt server, retrying...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    client
        .connect(mqtt::ConnectOptions::new())
        .await
        .map(|_| client)
}

pub fn influx_client() -> influxdb::Client {
    let auth = get_influx_auth();
    influx::Client::new("http://".to_string() + &get_influx_host() + ":8086", "iot")
        .with_auth(auth.0, auth.1)
}

pub fn init_logger() {
    if let Ok(_) = env::var("DEBUG_DATA_FLOW") {
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    }
}
