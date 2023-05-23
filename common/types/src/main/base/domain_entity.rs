use crate::main::base::value_object::ValueObject;
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(new, Clone, Default, Derivative, Serialize, Deserialize)]
#[derivative(PartialEq, Debug)]
pub struct DomainEntity<T, E> {
    pub id: T,
    pub version: Version,
    #[new(value = "vec![]")]
    #[derivative(PartialEq = "ignore")]
    #[derivative(Debug = "ignore")]
    pub events: Vec<E>,
}

pub trait DomainEntityTrait<E> {
    fn add_event(&mut self, event: E);
    fn pop_events(&self) -> &Vec<E>;
}

impl<E, T> DomainEntityTrait<E> for DomainEntity<T, E> {
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
