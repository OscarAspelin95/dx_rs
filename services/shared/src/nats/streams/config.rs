use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;
use async_nats::jetstream::consumer::{AckPolicy, DeliverPolicy};
use async_nats::jetstream::stream::DiscardPolicy;
use async_nats::{self, jetstream::stream::Config as StreamConfig};

pub enum StreamType {
    FileUpload,
}

pub struct StreamConsumerConfig {
    pub stream: StreamConfig,
    pub consumer: PushConsumerConfig,
}

impl From<StreamType> for StreamConsumerConfig {
    fn from(stream_type: StreamType) -> Self {
        match stream_type {
            StreamType::FileUpload => Self {
                stream: StreamConfig {
                    name: "file-uploaded".into(),
                    max_messages: 1_000,
                    subjects: vec!["file-uploaded.*".into()],
                    discard: DiscardPolicy::Old,
                    ..Default::default()
                },
                consumer: PushConsumerConfig {
                    name: Some("file-uploaded-process".into()),
                    durable_name: Some("file-uploaded-process".into()),
                    filter_subject: "file-uploaded.process".into(),
                    deliver_subject: "file-uploaded.process.deliver".into(),
                    ack_policy: AckPolicy::Explicit,
                    deliver_policy: DeliverPolicy::All,
                    ..Default::default()
                },
            },
        }
    }
}
