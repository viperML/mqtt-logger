mod logging;

use anyhow::Context;
use mqtt::{create_options, AsyncClient};
use paho_mqtt as mqtt;
use std::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crate::logging::setup_logging(log::LevelFilter::Info)?;
    let connect_options = mqtt::ConnectOptionsBuilder::new()
        .user_name("guest")
        .password("hello-world-hello-world")
        .finalize();

    // let client = mqtt::AsyncClient::new(connect_options)?;
    let client = mqtt::CreateOptionsBuilder::new()
        .client_id("mqtt-client")
        .server_uri("tcp://mqtt.ayats.org:1883")
        .create_client()
        .context("Error while creating the client")?;

    client.connect(connect_options).await?;

    let msg = mqtt::Message::new("test", "Hello World!", mqtt::QOS_1);

    let x = client.publish(msg).await?;

    client.disconnect(None).await?;

    println!("Hello world!");

    Ok(())
}
