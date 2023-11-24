use std::fmt::Debug;

use kafka::producer::{Producer, Record};

use derive_new::new;

use common_events::main::domain_event_publisher::DomainEventPublisher;
use domain::main::menu::meal_events::DomainEventEnum;

#[derive(new)]
pub struct KafkaEventPublisherImpl {
    topic_name: String,
    pub producer: Producer,
}

impl DomainEventPublisher<DomainEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<DomainEventEnum>) {
        let mut error_list = vec![];
        for event in events {
            let event_val: String = serde_json::to_string(event).unwrap();
            let msg = Record::from_value("meals", event_val.as_bytes());
            let result = self.producer.send(&msg);
            if result.is_err() {
                error_list.push(event_val);
            }
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
