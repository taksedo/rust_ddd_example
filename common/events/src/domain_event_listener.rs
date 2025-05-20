use std::{fmt::Debug, mem::Discriminant};

use async_trait::async_trait;

#[async_trait]
pub trait DomainEventListener<Event: Clone>: Debug + Send {
    fn event_type(&self) -> Discriminant<Event>;
    async fn handle(&mut self, event: &Event);
    fn get_events(&self) -> &Vec<Event>;
}
