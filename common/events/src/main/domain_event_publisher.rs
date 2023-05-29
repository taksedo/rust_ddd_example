use std::fmt::Debug;

pub trait DomainEventPublisher<E>: Debug + Send {
    #[allow(clippy::ptr_arg)]
    fn publish(&mut self, events: &Vec<E>);
}
