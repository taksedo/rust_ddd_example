use crate::main::base::domain_event::DomainEventTrait;
use crate::main::base::value_object::ValueObject;
use derive_new::new;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(new, Debug, Clone, Default)]
pub struct DomainEntity<I, E> {
    pub id: I,
    #[new(value = "vec![]")]
    pub events: Vec<E>,
}

pub trait DomainEntityTrait<E: DomainEventTrait>: Clone {
    fn add_event(&mut self, event: E);
    fn pop_events(&self) -> &Vec<E>;
}

impl<I: Clone, E: DomainEventTrait + Clone> DomainEntityTrait<E> for DomainEntity<I, E> {
    fn add_event(&mut self, event: E) {
        self.events.push(event)
    }
    fn pop_events(&self) -> &Vec<E> {
        &self.events
    }
}

#[derive(new, Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Version {
    #[new(value = "0_i64")]
    value: i64,
}

impl Version {
    pub fn increment(&self) -> Version {
        Self {
            value: &self.value + 1,
        }
    }
}

impl From<i64> for Version {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl ValueObject for Version {}
