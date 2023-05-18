use crate::main::base::domain_event::DomainEventTrait;
use crate::main::base::value_object::ValueObject;
use derivative::Derivative;
use derive_new::new;
use serde::Deserialize;
use serde::Serialize;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(new, Clone, Default, Derivative)]
#[derivative(PartialEq, Debug)]
pub struct DomainEntity<T> {
    pub id: T,
    pub version: Version,
    #[new(value = "vec![]")]
    #[derivative(PartialEq = "ignore")]
    #[derivative(Debug = "ignore")]
    pub events: Vec<Rc<RefCell<dyn DomainEventTrait>>>,
}

pub trait DomainEntityTrait {
    fn add_event(&mut self, event: Rc<RefCell<dyn DomainEventTrait>>);
    fn pop_events(&self) -> &Vec<Rc<RefCell<dyn DomainEventTrait>>>;
}

impl<T> DomainEntityTrait for DomainEntity<T> {
    fn add_event(&mut self, event: Rc<RefCell<dyn DomainEventTrait>>) {
        self.events.push(event)
    }
    fn pop_events(&self) -> &Vec<Rc<RefCell<dyn DomainEventTrait>>> {
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
