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
    pub events: Vec<E>,
}

pub trait DomainEntityTrait<E> {
    fn add_event(&mut self, event: E);
    fn pop_events(&mut self) -> Vec<E>;
}

impl<E: Clone, T> DomainEntityTrait<E> for DomainEntity<T, E> {
    fn add_event(&mut self, event: E) {
        if self.events.is_empty() {
            self.version = self.version.next();
        }
        self.events.push(event)
    }
    fn pop_events(&mut self) -> Vec<E> {
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
    pub fn next(&self) -> Version {
        Self {
            value: &self.value + 1,
        }
    }

    pub fn previous(&self) -> Version {
        Self {
            value: &self.value - 1,
        }
    }

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
