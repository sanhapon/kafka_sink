use rdkafka::config::ClientConfig;
use std::boxed::Box;

pub fn get_config() -> Result<(String, ClientConfig), Box<dyn std::error::Error>> {

    let mut kafka_config = ClientConfig::new();
    kafka_config.set("bootstrap.servers", "localhost:29092");

    Ok(
        (String::from("generic-message"), kafka_config,)
    )
}