mod util;

use std::collections::HashMap;
use std::io::Result;
use std::sync::Arc;

use log;
use tokio::sync::Mutex;
use influxdb::{InfluxDbWriteable, Timestamp};


type Payload = HashMap<String, serde_json::Value>;

#[tokio::main]
async fn main() -> Result<()> {
    util::init_logger();

    let mut mqtt_client = util::mqtt_client(5)
        .await
        .expect("could not connect to mqtt server");

    log::info!("connected to mqtt broker");

    let influx = Arc::new(Mutex::new(util::influx_client()));

    let event_stream = mqtt_client.get_stream(4096);

    mqtt_client.subscribe("#", 1).await?;

    while let Ok(message) = event_stream.recv().await {
        if let Some(msg) = message {
            let prefix = msg.topic().clone().replace("/", ".");
            let payload = serde_json::from_slice::<Payload>(msg.payload()).unwrap_or_default();

            log::info!("Received a message by topic [{}]", msg.topic());

            let timestamp: chrono::DateTime<chrono::Utc> =
                if let Some((value, true)) = payload.get("timestamp").map(|v| (v, v.is_string())) {
                    log::info!("Data timestamp is: {}", value);
                    chrono::DateTime::parse_from_rfc3339(value.as_str().unwrap())
                        .map(|t| t.into())
                        .unwrap_or(chrono::Utc::now())
                } else {
                    log::info!("Data timestamp is: NOW");
                    chrono::Utc::now()
                };

            let mut query = Timestamp::from(timestamp).into_query("iot");

            for (key, value) in payload.iter() {
                if value.is_number() {
                    let key = format!("{}.{}", prefix, key);
                    log::info!("{} {}", key, value);

                    if value.is_i64() {
                        query = query.add_field(key, value.as_i64());
                    } else if value.is_f64() {
                        query = query.add_field(key, value.as_f64());
                    }
                }
            }

            let influx = Arc::clone(&influx);
            tokio::spawn(async move {
                influx.lock().await.query(query).await.unwrap();
            });
        }
    }

    Ok(())
}
