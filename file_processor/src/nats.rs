use log::info;

use async_nats::jetstream::consumer::Consumer as PushConsumer;
use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;

pub async fn connect_nats() -> PushConsumer<PushConsumerConfig> {
    // Initialize NATS client.
    info!("Establishing NATS client...");
    let client =
        async_nats::connect(std::env::var("NATS_URL").expect("Missing environment variable."))
            .await
            .expect("Failed to connect to client.");

    // Enable JetStream.
    info!("Enabling JetStream...");
    let jetstream = async_nats::jetstream::new(client);

    info!("Getting stream...");
    let stream = jetstream
        .get_stream("file-uploaded")
        .await
        .expect("Failed to get stream.");

    info!("Getting consumer...");
    let consumer: PushConsumer<PushConsumerConfig> = stream
        .get_consumer("file-uploaded-process")
        .await
        .expect("Failed to get consumer");

    consumer
}
