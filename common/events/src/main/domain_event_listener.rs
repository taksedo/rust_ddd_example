use std::fmt::Debug;
use std::mem::Discriminant;

pub trait DomainEventListener<E: Clone>: Debug {
    fn event_type(&self) -> Discriminant<E>;
    fn handle(&mut self, event: &E);
    fn get_events(&self) -> &Vec<E>;
}
