use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use domain::main::menu::meal_events::DomainEventEnum;

use crate::main::event::event_publisher_impl::EventPublisherImpl;

lazy_static! {
    pub static ref EVENT_PUBLISHER: Arc<Mutex<EventPublisherImpl<DomainEventEnum>>> =
        Arc::new(Mutex::new(EventPublisherImpl::default()));
}
