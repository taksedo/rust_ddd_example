use std::{fmt::Debug, mem::Discriminant};

pub trait DomainEventListener<Event: Clone>: Debug + Send {
    fn event_type(&self) -> Discriminant<Event>;
    fn handle(&mut self, event: &Event);
    fn get_events(&self) -> &Vec<Event>;
}
