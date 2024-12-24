use std::fmt::Debug;

use types::base::DomainEventTrait;

pub trait DomainEventPublisher<Event: DomainEventTrait>: Debug + Send {
    #[allow(clippy::ptr_arg)]
    fn publish(&mut self, events: &Vec<Event>);
}
