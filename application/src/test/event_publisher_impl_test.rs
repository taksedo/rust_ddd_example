use crate::main::event::event_publisher_impl::EventPublisherImpl;
use common_events::main::domain_event_listener::DomainEventListener;
use common_events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use smart_default::SmartDefault;
use std::fmt::Debug;
use std::mem::{discriminant, Discriminant};
use std::sync::{Arc, Mutex};

#[test]
fn publish_events() {
    let mut publisher = EventPublisherImpl::default();

    let test_event_listener = TestEventListener::default();
    publisher.register_listener(test_event_listener.to_owned());

    let another_test_event_listener = AnotherTestEventListener::default();
    publisher.register_listener(another_test_event_listener.clone());

    let test_event: DomainEventEnum =
        DomainEventEnum::TestEvent(TestEvent::new("TestEvent".to_string()));
    let another_test_event: DomainEventEnum =
        DomainEventEnum::AnotherTestEvent(AnotherTestEvent::new("AnotherTestEvent".to_string()));
    let events: Vec<DomainEventEnum> = vec![test_event.clone(), another_test_event.clone()];

    publisher.publish(&events);

    let test_event_listener = &publisher
        .get_listener(DomainEventEnum::TestEvent(TestEvent::default()))
        .lock()
        .unwrap();

    let another_test_event_listener = &publisher
        .get_listener(DomainEventEnum::AnotherTestEvent(
            AnotherTestEvent::default(),
        ))
        .lock()
        .unwrap();

    assert_eq!(test_event_listener.get_events(), &vec![test_event]);
    assert_eq!(
        another_test_event_listener.get_events(),
        &vec![another_test_event]
    );
}

#[derive(new, Default, Debug, Clone, PartialEq)]
struct TestEventListener {
    pub events: Vec<DomainEventEnum>,
}

impl DomainEventListener<DomainEventEnum> for TestEventListener {
    fn event_type(&self) -> Discriminant<DomainEventEnum> {
        let event: DomainEventEnum = (TestEvent::default()).into();
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
        let event: DomainEventEnum = (AnotherTestEvent::default()).into();
        discriminant(&event)
    }

    fn handle(&mut self, event: &DomainEventEnum) {
        self.events.push(event.to_owned());
    }
    fn get_events(&self) -> &Vec<DomainEventEnum> {
        &self.events
    }
}

#[enum_dispatch(DomainEventTrait)]
#[derive(Debug, Clone, PartialEq, Hash, Eq, SmartDefault)]
enum DomainEventEnum {
    #[default]
    TestEvent(TestEvent),
    AnotherTestEvent(AnotherTestEvent),
}

// impl Default for DomainEventEnum {
//     fn default() -> Self {
//         Self::TestEvent(TestEvent::default())
//     }
// }

#[enum_dispatch]
trait DomainEventTrait {}

#[derive(new, Debug, Clone, Default, PartialEq, Hash, Eq)]
struct TestEvent {
    name: String,
}

impl DomainEventTrait for TestEvent {}

#[derive(new, Debug, Clone, Default, PartialEq, Hash, Eq)]
struct AnotherTestEvent {
    name: String,
}

impl DomainEventTrait for AnotherTestEvent {}

impl<E: Debug> EventPublisherImpl<E> {
    fn get_listener(&self, event_type: E) -> &Arc<Mutex<dyn DomainEventListener<E>>> {
        let result = self.listener_map.get(&discriminant(&event_type)).unwrap();
        result.get(0).unwrap()
    }
}
