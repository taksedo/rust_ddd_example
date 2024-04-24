use std::sync::{Arc, Mutex};

use common::types::base::generic_types::AM;
use lazy_static::lazy_static;

use crate::event::kafka_event_publisher_impl::KafkaEventPublisherImpl;

// lazy_static! {
//     pub static ref EVENT_PUBLISHER: AM<EventPublisherImpl<DomainEventEnum>> =
//         Arc::new(Mutex::new(EventPublisherImpl::default()));
// }

lazy_static! {
    pub(super) static ref EVENT_PUBLISHER: AM<KafkaEventPublisherImpl> =
        Arc::new(Mutex::new(KafkaEventPublisherImpl::default()));
}
