use std::{env, fmt::Debug, time::Duration};

use common::events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use domain::main::{
    menu::meal_events::MealEventEnum, order::customer_order_events::ShopOrderEventEnum,
};
use kafka::producer::{Producer, Record, RequiredAcks};

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

impl DomainEventPublisher<MealEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<MealEventEnum>) {
        events.iter().for_each(|event| {
            let event_serialized: String = serde_json::to_string(event).unwrap();
            let msg = Record::from_value(MEAL_TOPIC_NAME, event_serialized.as_bytes());
            self.producer
                .send(&msg)
                .expect("Something is wrong with sending to Kafka");
        })
    }
}

impl DomainEventPublisher<ShopOrderEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<ShopOrderEventEnum>) {
        events.iter().for_each(|event| {
            let event_serialized: String = serde_json::to_string(event).unwrap();
            let msg = Record::from_value(ORDER_TOPIC_NAME, event_serialized.as_bytes());
            self.producer
                .send(&msg)
                .expect("Something is wrong with sending to Kafka");
        })
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
pub const ORDER_TOPIC_NAME: &str = "order_topic";
