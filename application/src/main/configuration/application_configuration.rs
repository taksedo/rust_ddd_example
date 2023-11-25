use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::main::event::kafka_event_publisher_impl::KafkaEventPublisherImpl;

// lazy_static! {
//     pub static ref EVENT_PUBLISHER: Arc<Mutex<EventPublisherImpl<DomainEventEnum>>> =
//         Arc::new(Mutex::new(EventPublisherImpl::default()));
// }

lazy_static! {
    pub static ref EVENT_PUBLISHER: Arc<Mutex<KafkaEventPublisherImpl>> =
        Arc::new(Mutex::new(KafkaEventPublisherImpl::default()));
}
