use std::{ops::Deref, time::Duration};

use derive_new::new;
use kafka::{
    consumer::{Consumer, FetchOffset, GroupOffsetStorage},
    producer::{Producer, Record, RequiredAcks},
};

use domain::{
    main::menu::{
        meal_events::{DomainEventEnum, MealAddedToMenuDomainEvent},
        value_objects::meal_id::{MealId, MealIdGenerator},
    },
    test_fixtures::fixtures::rnd_meal_id,
};

use crate::main::event::kafka_event_publisher_impl::MEAL_TOPIC_NAME;
use crate::{
    main::event::kafka_event_publisher_impl::KafkaEventPublisherImpl,
    test_fixtures::{TestKafka, KAFKA_ADDRESS},
};

#[tokio::test]
async fn publish_events() {
    let _container = TestKafka::new().await;

    let mut id_generator = TestMealIdGenerator::new(rnd_meal_id());
    let events_enum = vec![DomainEventEnum::MealAddedToMenuDomainEvent(
        MealAddedToMenuDomainEvent::new(id_generator.generate()),
    )];

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
        .unwrap();

    let receiver = MockReceiver::new(
        MEAL_TOPIC_NAME.to_string(),
        "My Test Group".to_string(),
        Some(GroupOffsetStorage::Kafka),
    );
    let a = receiver.receive().unwrap();
    assert_eq!(a.last().unwrap(), &buffer)
}

#[derive(Debug, new, Default)]
pub(crate) struct TestMealIdGenerator {
    pub meal_id: MealId,
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
    pub fn receive(&self) -> Result<Vec<String>, &str> {
        let topic_name = self.topic_name.clone();

        let mut consumer = Consumer::from_hosts(vec![KAFKA_ADDRESS.get().unwrap().to_owned()])
            .with_topic(topic_name.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(self.test_group.clone())
            .with_offset_storage(self.storage.clone())
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
