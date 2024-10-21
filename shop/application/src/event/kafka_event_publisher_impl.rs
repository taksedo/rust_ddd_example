use std::{
    env,
    fmt::{Debug, Formatter},
};

use common::events::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use domain::{menu::meal_events::MealEventEnum, order::customer_order_events::ShopOrderEventEnum};
use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

#[derive(new)]
pub(crate) struct KafkaEventPublisherImpl {
    topic_name: String,
    producer: BaseProducer,
}

impl Debug for KafkaEventPublisherImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafkaEventPublisherImpl")
            .field("topic_name", &self.topic_name)
            .field("producer", &&"...")
            .finish()
    }
}

pub(crate) const MEAL_TOPIC_NAME: &str = "meal_topic";
pub(super) const ORDER_TOPIC_NAME: &str = "order_topic";

impl DomainEventPublisher<MealEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<MealEventEnum>) {
        events.iter().for_each(|event| {
            let event_serialized: String = serde_json::to_string(event).unwrap();
            let msg = BaseRecord::to(MEAL_TOPIC_NAME)
                .key(&[1, 2, 3, 4])
                .payload(&event_serialized);
            self.producer
                .send(msg)
                .expect("Something is wrong with sending to Kafka");
        })
    }
}

impl DomainEventPublisher<ShopOrderEventEnum> for KafkaEventPublisherImpl {
    fn publish(&mut self, events: &Vec<ShopOrderEventEnum>) {
        events.iter().for_each(|event| {
            let event_serialized: String = serde_json::to_string(event).unwrap();
            let msg = BaseRecord::to(ORDER_TOPIC_NAME)
                .key(&[1, 2, 3, 4])
                .payload(&event_serialized);
            self.producer
                .send(msg)
                .expect("Something is wrong with sending to Kafka");
        })
    }
}

impl Default for KafkaEventPublisherImpl {
    fn default() -> Self {
        let kafka_address = env::var("KAFKA_ADDRESS")
            .unwrap()
            .parse::<String>()
            .unwrap();
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_address)
            .create()
            .expect("Producer creation error");
        Self {
            topic_name: MEAL_TOPIC_NAME.to_owned(),
            producer,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, time::Duration};

    use domain::menu::{
        meal_events::MealAddedToMenuDomainEvent,
        value_objects::meal_id::{MealId, MealIdGenerator},
    };
    use futures_lite::StreamExt;
    use rdkafka::{
        consumer::{Consumer, StreamConsumer},
        Message,
    };
    use tracing::{debug, info};

    use super::*;
    use crate::{
        domain_test_fixtures::rnd_meal_id,
        test_fixtures::{TestKafka, KAFKA_ADDRESS},
    };

    #[tokio::test]
    async fn publish_events() {
        let _container = TestKafka::new().await;

        let mut id_generator = TestMealIdGenerator::new(rnd_meal_id());
        let events_enum: Vec<MealEventEnum> =
            vec![MealAddedToMenuDomainEvent::new(id_generator.generate()).into()];

        let kafka_address = KAFKA_ADDRESS.get().unwrap();
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_address)
            .create()
            .expect("Producer creation error");

        let topic_name = MEAL_TOPIC_NAME;

        let mut publisher = KafkaEventPublisherImpl::new(topic_name.to_owned(), producer);
        publisher.publish(&events_enum);

        let receiver = MockReceiver::new(
            MEAL_TOPIC_NAME.to_string(),
            "My Test Group".to_string(),
            // Some(GroupOffsetStorage::Kafka),
        );

        let result = receiver.receive().await.unwrap();
        info!(?result);
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

    struct MockReceiver {
        consumer: StreamConsumer,
        topic_name: String,
    }

    impl MockReceiver {
        fn new(topic_name: String, test_group: String) -> Self {
            let consumer = ClientConfig::new()
                .set("group.id", test_group.clone())
                .set("bootstrap.servers", KAFKA_ADDRESS.get().unwrap())
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "false")
                .set("auto.offset.reset", "earliest")
                .create::<StreamConsumer>()
                .expect("Failed to create Kafka StreamConsumer");

            debug!("Consumer created for {test_group}");

            consumer
                .subscribe(&[&topic_name])
                .expect("Unable to subscribe to {self.topic_name}");
            debug!("Consumer subscribed to {topic_name}");

            Self {
                consumer,
                topic_name,
            }
        }

        async fn receive(&self) -> Result<Vec<String>, Box<dyn Error + 'static>> {
            let mut result = vec![];
            debug!("Waiting for messages on {}", &self.topic_name);

            let mut message_stream = self.consumer.stream();

            let borrowed_message =
                tokio::time::timeout(Duration::from_secs(10), message_stream.next())
                    .await
                    .unwrap()
                    .unwrap();

            let result_str = borrowed_message
                .unwrap()
                .payload_view::<str>()
                .unwrap()
                .unwrap()
                .to_owned();

            result.push(result_str);

            Ok(result.clone())
        }
    }
}
