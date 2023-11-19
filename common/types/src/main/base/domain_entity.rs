use crate::main::base::value_object::ValueObject;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(new, Clone, Default, Derivative, Serialize, Deserialize)]
#[derivative(PartialEq, Debug)]
pub struct DomainEntity<T, Event> {
    pub id: T,
    pub version: Version,
    #[new(value = "vec![]")]
    #[derivative(PartialEq = "ignore")]
    pub events: Vec<Event>,
}

pub trait DomainEntityTrait<Event> {
    fn add_event(&mut self, event: Event);
    fn pop_events(&mut self) -> Vec<Event>;
}

impl<Event: Clone, T> DomainEntityTrait<Event> for DomainEntity<T, Event> {
    fn add_event(&mut self, event: Event) {
        if self.events.is_empty() {
            self.version = self.version.next();
        }
        self.events.push(event)
    }
    fn pop_events(&mut self) -> Vec<Event> {
        let res = self.events.clone();
        self.events = Vec::new();
        res
    }
}

#[derive(new, Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Version {
    #[new(value = "0_i64")]
    value: i64,
}

impl Version {
    /// Returns next `Version`
    pub fn next(&self) -> Version {
        Self {
            value: &self.value + 1,
        }
    }

    /// Returns previous `Version`
    pub fn previous(&self) -> Version {
        Self {
            value: &self.value - 1,
        }
    }

    /// Converts `Version` to `i64`
    pub fn to_i64(&self) -> i64 {
        self.value
    }
}

impl From<i64> for Version {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl ValueObject for Version {}
