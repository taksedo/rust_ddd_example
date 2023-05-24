use std::fmt::Debug;

pub trait DomainEventPublisher<E>: Debug + Send {
    fn publish(&mut self, events: &Vec<E>);
}
