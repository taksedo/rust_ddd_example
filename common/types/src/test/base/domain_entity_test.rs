use std::{any::Any, fmt::Debug};

use derive_new::new;

use crate::main::base::{
    domain_entity::{DomainEntity, DomainEntityTrait, Version},
    domain_event::DomainEventTrait,
};

#[test]
#[allow(non_snake_case)]
fn produce_event__event_stuck_is_clean() {
    let id = 1_i64;
    let version = Version::default();

    let mut entity = TestEntity::new(DomainEntity::new(id, version));

    entity.do_something();

    assert_eq!(entity.domain_entity_field.id.clone(), id.clone());
    assert_eq!(entity.domain_entity_field.version, version.next());
    let events = entity.pop_events();
    assert_eq!(&events.len(), &1);

    assert_eq!(
        events.get(0).unwrap().clone().type_id(),
        TestEvent::new().type_id()
    );
}

#[test]
fn version_is_incremented_only_single_times_after_altering_entity() {
    let id = 1_i64;
    let version = Version::default();

    let mut entity = TestEntity::new(DomainEntity::new(id, version));

    for _ in 0..10 {
        entity.do_something();
    }

    assert_eq!(entity.domain_entity_field.version, version.next())
}

#[test]
fn version_is_incremented_after_popping_events() {
    let id = 1_i64;
    let version = Version::default();

    let mut entity = TestEntity::new(DomainEntity::new(id, version));
    entity.do_something();
    entity.pop_events();
    entity.do_something();
    assert_eq!(entity.domain_entity_field.version, version.next().next())
}

#[derive(new, Debug, Clone)]
struct TestEntity {
    pub domain_entity_field: DomainEntity<i64, TestEvent>,
}

impl TestEntity {
    pub fn do_something(&mut self) {
        self.add_event(TestEvent::new())
    }
}

impl DomainEntityTrait<TestEvent> for TestEntity {
    fn add_event(&mut self, event: TestEvent) {
        if self.domain_entity_field.events.is_empty() {
            self.domain_entity_field.version = self.domain_entity_field.version.next();
        }
        self.domain_entity_field.events.push(event)
    }

    fn pop_events(&mut self) -> Vec<TestEvent> {
        let res = self.domain_entity_field.events.clone();
        self.domain_entity_field.events = Vec::new();
        res
    }
}

#[derive(new, Debug, Clone)]
struct TestEvent {}

impl DomainEventTrait for TestEvent {}
