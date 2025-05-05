#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    mem::{Discriminant, discriminant},
};

use common::{
    events::{DomainEventListener, DomainEventPublisher},
    types::base::{AM, AMTrait, DomainEventTrait},
};
use derive_new::new;
use log::info;

type VecOfDomainEventListenerType<Event> = Vec<AM<dyn DomainEventListener<Event>>>;

#[derive(new, Debug, Default, Clone)]
pub(crate) struct EventPublisherImpl<Event: Debug> {
    pub(crate) listener_map: HashMap<Discriminant<Event>, VecOfDomainEventListenerType<Event>>,
}

impl<Event: Debug + Clone + Hash + Eq> EventPublisherImpl<Event> {
    fn register_listener(&mut self, listener: impl DomainEventListener<Event> + 'static) {
        let event_type = listener.event_type();
        self.listener_map.entry(event_type).or_insert_with(|| {
            let vector: Vec<AM<(dyn DomainEventListener<Event> + 'static)>> =
                vec![AM::new_am(listener)];
            vector
        });
    }

    fn send_events(&self, listeners: Vec<AM<dyn DomainEventListener<Event>>>, event: Event) {
        listeners.iter().for_each(|l| l.lock_un().handle(&event))
    }
}

impl<Event> DomainEventPublisher<Event> for EventPublisherImpl<Event>
where
    Event: Debug + Clone + 'static + Hash + Eq + Default + DomainEventTrait,
{
    fn publish(&mut self, events: &Vec<Event>) {
        events.iter().for_each(|e| {
            info!("Processing event: {:?}", &e);
            let listener_map = &self.listener_map;
            let e_type = discriminant(e);
            if listener_map.contains_key(&e_type) {
                let listeners_from_listener_map = listener_map.get(&e_type).unwrap();
                self.send_events(listeners_from_listener_map.to_vec(), e.clone())
            }
        })
    }
}

#[cfg(test)]
mod test {
    use common::types::base::DomainEventTrait;
    use smart_default::SmartDefault;

    use super::*;
    #[test]
    fn publish_events() {
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

        publisher.publish(&events);

        let test_event_listener = &publisher
            .get_listener(DomainEventEnum::TestEvent(TestEvent::default()))
            .lock_un();

        let another_test_event_listener = &publisher
            .get_listener(DomainEventEnum::AnotherTestEvent(
                AnotherTestEvent::default(),
            ))
            .lock_un();

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

    impl DomainEventListener<DomainEventEnum> for TestEventListener {
        fn event_type(&self) -> Discriminant<DomainEventEnum> {
            let event: DomainEventEnum = TestEvent::default().into();
            discriminant(&event)
        }

        fn handle(&mut self, event: &DomainEventEnum) {
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

    impl DomainEventListener<DomainEventEnum> for AnotherTestEventListener {
        fn event_type(&self) -> Discriminant<DomainEventEnum> {
            let event: DomainEventEnum = AnotherTestEvent::default().into();
            discriminant(&event)
        }

        fn handle(&mut self, event: &DomainEventEnum) {
            self.events.push(event.to_owned());
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
        fn get_listener(&self, event_type: Event) -> &AM<dyn DomainEventListener<Event>> {
            let result = self.listener_map.get(&discriminant(&event_type)).unwrap();
            result.first().unwrap()
        }
    }
}
