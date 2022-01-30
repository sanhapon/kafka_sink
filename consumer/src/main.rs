// https://docs.rs/kafka/latest/kafka/consumer/index.html

use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::consumer::CommitMode;
use std::boxed::Box;

mod utils;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (topic, mut config) = utils::get_config()?;

    println!("start listening for {:?}", topic);
    let result = config.set("group.id", "rust_example_group_1").create::<StreamConsumer>();

    match result {
        Ok(consumer) => {
            println!("subscribe");
            consumer.subscribe(&vec![topic.as_ref()])?;

            loop {
                let _ = match consumer.recv().await {
                    Ok(msg) => {
                        let bytes = msg.payload().unwrap();

                        let msg: proto::GenericMessage = protobuf::Message::parse_from_bytes(&bytes[6..]).unwrap();
                        println!("{:?}", msg);

                        consumer.commit_consumer_state(CommitMode::Sync).unwrap();

                    },
                    Err(_) => break,
                };
            }
        },
        Err(err) => {
            println!("error ==> {}", err)
        }
    };

    Ok(())
}
