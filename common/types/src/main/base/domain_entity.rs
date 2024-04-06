use std::fmt::Debug;

use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::main::base::value_object::ValueObject;

/// Abstract class for all `Entities` and `Aggregates`.
#[derive(new, Clone, Default, Derivative, Serialize, Deserialize, Ord, PartialOrd, Eq)]
#[derivative(PartialEq, Debug)]
pub struct DomainEntity<T, Event> {
    pub id: T,
    pub version: Version,
    #[new(value = "vec![]")]
    #[derivative(PartialEq = "ignore")]
    pub events: Vec<Event>,
}

pub trait DomainEntityTrait<Event> {
    /// Add `Event` to a struct
    fn add_event(&mut self, event: Event);
    /// Extract all `Events` from a struct's stack
    fn pop_events(&mut self) -> Vec<Event>;
}

impl<Event: Clone, T> DomainEntityTrait<Event> for DomainEntity<T, Event> {
    /// Add `Event` to `DomainEntity` stack
    fn add_event(&mut self, event: Event) {
        if self.events.is_empty() {
            self.version = self.version.next();
        }
        self.events.push(event)
    }
    /// Extract all `Events` from `DomainEntity` stack
    fn pop_events(&mut self) -> Vec<Event> {
        let res = self.events.clone();
        self.events = Vec::new();
        res
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Version(i64);

impl Version {
    /// Returns next `Version`
    pub fn next(&self) -> Self {
        Version(&self.0 + 1)
    }

    /// Returns previous `Version`
    pub fn previous(&self) -> Self {
        Self(&self.0 - 1)
    }

    /// Converts `Version` to `i64`
    pub fn to_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for Version {
    /// Gets `Version` from `i64`
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl ValueObject for Version {}

#[cfg(test)]
mod domain_entity_test {
    use std::{any::Any, fmt::Debug};

    use derive_new::new;

    use super::*;
    use crate::main::base::domain_event::DomainEventTrait;

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
            events.first().unwrap().clone().type_id(),
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
}

#[cfg(test)]
mod version_test {
    use std::convert::From;

    use rand::random;

    use super::*;

    #[test]
    fn new_id_check_version_is_zero() {
        let first_version = Version::default();
        let second_version = Version::default();

        assert_eq!(first_version.to_i64(), second_version.to_i64());
        assert_eq!(first_version.to_i64(), 0)
    }

    #[test]
    fn restore_from_long() {
        let long: i64 = random();
        let version = Version::from(long);
        assert_eq!(version.to_i64(), long)
    }

    #[test]
    fn increment_counter_value_is_plus_1() {
        let long: i64 = random();
        let version = Version::from(long);
        let incremented = version.next();
        assert_eq!(incremented.to_i64(), long + 1)
    }

    #[test]
    fn the_same_value_should_be_equals() {
        let long: i64 = random();
        let first = Version::from(long);
        let second = Version::from(long);
        assert_eq!(first, second)
    }

    #[test]
    fn previous_version_should_be_current_minus_1() {
        let long: i64 = random();
        let version = Version::from(long);
        let previous = version.previous();
        assert_eq!(previous.to_i64(), version.to_i64() - 1)
    }
}
