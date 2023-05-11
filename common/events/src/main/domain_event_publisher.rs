use common_types::main::base::domain_event::DomainEventTrait;
use std::fmt::Debug;

pub trait DomainEventPublisher<E: DomainEventTrait>: Debug + Clone {
    fn publish(&mut self, events: &Vec<E>);
}
