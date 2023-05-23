use std::fmt::Debug;

pub trait DomainEventPublisher<E>: Debug {
    fn publish(&mut self, events: &Vec<E>);
}
