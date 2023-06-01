use crate::main::event::event_publisher_impl::EventPublisherImpl;
use domain::main::menu::meal_events::DomainEventEnum;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref EVENT_PUBLISHER: Arc<Mutex<EventPublisherImpl<DomainEventEnum>>> =
        Arc::new(Mutex::new(EventPublisherImpl::default()));
}
