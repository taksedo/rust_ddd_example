use common_types::main::base::domain_event::DomainEventTrait;
use std::fmt::Debug;
use std::rc::Rc;

pub trait DomainEventPublisher: Debug {
    fn publish<T: DomainEventTrait>(&mut self, events: Vec<T>);
}
