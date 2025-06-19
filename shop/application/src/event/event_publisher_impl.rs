#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    mem::{Discriminant, discriminant},
};

use async_trait::async_trait;
use common::{
    events::{DomainEventListener, DomainEventPublisher},
    types::base::{AM, AMTrait, DomainEventTrait},
};
use derive_new::new;
use log::info;

type ListenerVec<Event> = Vec<AM<dyn DomainEventListener<Event> + Send>>;

#[derive(new, Debug, Default, Clone)]
pub(crate) struct EventPublisherImpl<Event: Debug> {
    listener_map: HashMap<Discriminant<Event>, ListenerVec<Event>>,
}

impl<Event> EventPublisherImpl<Event>
where
    Event: Debug + Clone + Hash + Eq,
{
    fn register_listener(&mut self, listener: impl DomainEventListener<Event> + 'static) {
        let event_type = listener.event_type();
        let entry = self.listener_map.entry(event_type).or_default();
        entry.push(AM::new_am(listener));
    }

    async fn send_events(
        &self,
        listeners: &[AM<dyn DomainEventListener<Event> + Send>],
        event: Event,
    ) {
        for l in listeners {
            l.lock().await.handle(&event).await;
        }
    }
}

#[async_trait]
impl<Event> DomainEventPublisher<Event> for EventPublisherImpl<Event>
where
    Event: Debug + Clone + 'static + Hash + Eq + Default + DomainEventTrait + Sync + Send,
{
    async fn publish(&mut self, events: &[Event]) {
        for event in events.iter() {
            info!("Processing event: {:?}", &event);
            let listener_map = &self.listener_map;
            let event_type = discriminant(event);
            if listener_map.contains_key(&event_type) {
                let listeners_from_listener_map = listener_map.get(&event_type).unwrap();
                self.send_events(listeners_from_listener_map, event.clone())
                    .await;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use common::types::base::DomainEventTrait;
    use smart_default::SmartDefault;
    use tokio::test;

    use super::*;
    #[test]
    async fn publish_events() {
        if std::env::var("RUST_LOG").is_err() {
            unsafe {
                std::env::set_var("RUST_LOG", "debug");
            }
        }
        let _ = tracing_subscriber::fmt::try_init();
        let mut publisher = EventPublisherImpl::default();

        let test_event_listener = TestEventListener::default();
        publisher.register_listener(test_event_listener);

        let another_test_event_listener = AnotherTestEventListener::default();
        publisher.register_listener(another_test_event_listener);

        let test_event: DomainEventEnum = TestEvent::new("TestEvent".to_string()).into();
        let another_test_event: DomainEventEnum =
            AnotherTestEvent::new("AnotherTestEvent".to_string()).into();
        let events: Vec<DomainEventEnum> = vec![test_event.clone(), another_test_event.clone()];

        publisher.publish(&events).await;

        let test_event_listener = &publisher
            .get_listener(DomainEventEnum::TestEvent(TestEvent::default()))
            .lock()
            .await;

        let another_test_event_listener = &publisher
            .get_listener(DomainEventEnum::AnotherTestEvent(
                AnotherTestEvent::default(),
            ))
            .lock()
            .await;

        assert_eq!(test_event_listener.get_events(), &vec![test_event]);
        assert_eq!(
            another_test_event_listener.get_events(),
            &vec![another_test_event]
        );
    }

    #[derive(new, Default, Debug, Clone, PartialEq)]
    struct TestEventListener {
        events: Vec<DomainEventEnum>,
    }

    #[async_trait]
    impl DomainEventListener<DomainEventEnum> for TestEventListener {
        fn event_type(&self) -> Discriminant<DomainEventEnum> {
            let event: DomainEventEnum = TestEvent::default().into();
            discriminant(&event)
        }

        async fn handle(&mut self, event: &DomainEventEnum) {
            self.events.push(event.clone());
        }

        fn get_events(&self) -> &Vec<DomainEventEnum> {
            &self.events
        }
    }

    #[derive(Default, Debug, new, Clone, PartialEq)]
    struct AnotherTestEventListener {
        events: Vec<DomainEventEnum>,
    }

    #[async_trait]
    impl DomainEventListener<DomainEventEnum> for AnotherTestEventListener {
        fn event_type(&self) -> Discriminant<DomainEventEnum> {
            let event: DomainEventEnum = AnotherTestEvent::default().into();
            discriminant(&event)
        }

        async fn handle(&mut self, event: &DomainEventEnum) {
            self.events.push(event.clone());
        }

        fn get_events(&self) -> &Vec<DomainEventEnum> {
            &self.events
        }
    }

    #[enum_delegate::implement(DomainEventTrait)]
    #[derive(Debug, Clone, PartialEq, Hash, Eq, SmartDefault)]
    enum DomainEventEnum {
        #[default]
        TestEvent(TestEvent),
        AnotherTestEvent(AnotherTestEvent),
    }

    #[derive(new, Debug, Clone, Default, PartialEq, Hash, Eq)]
    struct TestEvent {
        name: String,
    }

    #[derive(new, Debug, Clone, Default, PartialEq, Hash, Eq)]
    struct AnotherTestEvent {
        name: String,
    }

    impl<Event: Debug> EventPublisherImpl<Event> {
        fn get_listener(&self, event_type: Event) -> &AM<dyn DomainEventListener<Event> + Send> {
            let result = self.listener_map.get(&discriminant(&event_type)).unwrap();
            result.first().unwrap()
        }
    }
}
