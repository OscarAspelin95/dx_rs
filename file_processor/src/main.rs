use async_nats;
use async_nats::jetstream::consumer::Consumer as PushConsumer;
use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;
use futures::StreamExt;
use log::info;
use simple_logger::SimpleLogger;
use tokio;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");
    info!("Inside file processor.");

    // Initialize NATS client.
    info!("Establishing NATS client...");
    let client =
        async_nats::connect(std::env::var("NATS_URL").expect("Missing environment variable."))
            .await
            .expect("Failed to connect to client.");

    // Enable JetStream.
    info!("Enabling JetStream...");
    let jetstream = async_nats::jetstream::new(client);

    // Get file processor consumer.
    let mut subscriber = jetstream
        .client()
        .subscribe("file-uploaded.process.deliver")
        .await
        .expect("Failed to create subscriber");

    // Loop infinitely.
    info!("Ready to accept messages...");
    while let Some(message) = subscriber.next().await {
        info!("Got message!");

        // Bytes, need to convert with serde_json to appropriate data structure.
        let payload = &message.payload;

        info!("Payload: {:?}", payload);
        println!("{:?}", payload);
    }
}
