use std::{env, fmt::Debug, time::Duration};

use common::events::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use domain::main::{
    menu::meal_events::MealEventEnum, order::customer_order_events::ShopOrderEventEnum,
};
use kafka::producer::{Producer, Record, RequiredAcks};

#[derive(new)]
pub(crate) struct KafkaEventPublisherImpl {
    topic_name: String,
    producer: Producer,
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
            .field("producer", &&"...")
            .finish()
    }
}

pub(crate) const MEAL_TOPIC_NAME: &str = "meal_topic";
pub(super) const ORDER_TOPIC_NAME: &str = "order_topic";

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use domain::{
        main::menu::{
            meal_events::MealAddedToMenuDomainEvent,
            value_objects::meal_id::{MealId, MealIdGenerator},
        },
        test_fixtures::rnd_meal_id,
    };
    use kafka::{
        client::{FetchOffset, GroupOffsetStorage},
        consumer::Consumer,
    };

    use super::*;
    use crate::{
        event::kafka_event_publisher_impl::{KafkaEventPublisherImpl, MEAL_TOPIC_NAME},
        test_fixtures::{TestKafka, KAFKA_ADDRESS},
    };

    #[tokio::test]
    async fn publish_events() {
        let _container = TestKafka::new().await;

        let mut id_generator = TestMealIdGenerator::new(rnd_meal_id());
        let events_enum: Vec<MealEventEnum> =
            vec![MealAddedToMenuDomainEvent::new(id_generator.generate()).into()];

        let test_events_str = serde_json::to_string(&events_enum.first()).unwrap();
        let kafka_address = KAFKA_ADDRESS.get().unwrap();
        let producer = Producer::from_hosts(vec![kafka_address.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        let topic_name = MEAL_TOPIC_NAME;

        let mut publisher = KafkaEventPublisherImpl::new(topic_name.to_owned(), producer);

        let buffer = test_events_str;

        publisher
            .producer
            .send(&Record::from_value(topic_name, buffer.as_bytes()))
            .expect("Event send failed");
        let receiver = MockReceiver::new(
            topic_name.to_string(),
            "My Test Group".to_string(),
            Some(GroupOffsetStorage::Kafka),
        );
        let a = receiver.receive().unwrap();
        assert_eq!(a.last().unwrap(), &buffer)
    }

    #[derive(Debug, new, Default)]
    pub(crate) struct TestMealIdGenerator {
        meal_id: MealId,
    }

    impl MealIdGenerator for TestMealIdGenerator {
        fn generate(&mut self) -> MealId {
            self.meal_id
        }
    }

    #[derive(new)]
    struct MockReceiver {
        topic_name: String,
        test_group: String,
        storage: Option<GroupOffsetStorage>,
    }

    impl MockReceiver {
        fn receive(&self) -> Result<Vec<String>, &str> {
            let topic_name = self.topic_name.clone();

            let mut consumer = Consumer::from_hosts(vec![KAFKA_ADDRESS.get().unwrap().to_owned()])
                .with_topic(topic_name.to_owned())
                .with_fallback_offset(FetchOffset::Earliest)
                .with_group(self.test_group.clone())
                .with_offset_storage(self.storage)
                .create()
                .unwrap();
            let mut result = vec![];

            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    let str = String::from_utf8_lossy(m.value);
                    println!("{:?}", &str);
                    result.push(String::from(str.deref()));
                }
                let _ = consumer.consume_messageset(ms);
            }
            consumer.commit_consumed().unwrap();

            Ok(result)
        }
    }
}
