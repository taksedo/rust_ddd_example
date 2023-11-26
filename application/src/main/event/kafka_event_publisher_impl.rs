use std::env;
use std::fmt::Debug;
use std::time::Duration;

use derive_new::new;
use kafka::producer::{Producer, Record, RequiredAcks};

use common_events::main::domain_event_publisher::DomainEventPublisher;
use domain::main::menu::meal_events::DomainEventEnum;

#[derive(new)]
pub struct KafkaEventPublisherImpl {
    topic_name: String,
    pub producer: Producer,
}

impl Default for KafkaEventPublisherImpl {
    fn default() -> Self {
        let kafka_address = env::var("KAFKA_ADDRESS")
            .unwrap()
            .parse::<String>()
            .unwrap();
        let producer = Producer::from_hosts(vec![kafka_address])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();
        Self {
            topic_name: MEAL_TOPIC_NAME.to_owned(),
            producer,
        }
    }
}

impl DomainEventPublisher<DomainEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<DomainEventEnum>) {
        for event in events {
            let event_serialized: String = serde_json::to_string(event).unwrap();
            let msg = Record::from_value(MEAL_TOPIC_NAME, event_serialized.as_bytes());
            self.producer
                .send(&msg)
                .expect("Something is wrong with sending to Kafka");
        }
    }
}

impl Debug for KafkaEventPublisherImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafkaEventPublisherImpl")
            .field("topic_name", &self.topic_name)
            .field("producer", &&"..." as _)
            .finish()
    }
}

pub const MEAL_TOPIC_NAME: &str = "meal_topic";
