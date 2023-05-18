use common_types::main::base::domain_event::DomainEventTrait;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait DomainEventPublisher: Debug {
    fn publish(&mut self, events: &Vec<Rc<RefCell<dyn DomainEventTrait>>>);
}
