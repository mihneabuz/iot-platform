mod util;

use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mqtt_client = util::mqtt_client(0).await.expect("could not connect to mqtt server");
    println!("connected to mqtt broker");

    mqtt_client.subscribe("#", 1).await?;

    mqtt_client.set_message_callback(|client, message| {
        println!("{}", message.unwrap());
    });

    Ok(())
}
