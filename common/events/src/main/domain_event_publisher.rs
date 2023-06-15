use std::fmt::Debug;

pub trait DomainEventPublisher<Event>: Debug + Send {
    #[allow(clippy::ptr_arg)]
    fn publish(&mut self, events: &Vec<Event>);
}
