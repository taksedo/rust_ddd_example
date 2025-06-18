use std::fmt::Debug;

use async_trait::async_trait;
use types::base::DomainEventTrait;

#[async_trait]
pub trait DomainEventPublisher<Event: DomainEventTrait>: Debug + Send {
    async fn publish(&mut self, events: &[Event]);
}
