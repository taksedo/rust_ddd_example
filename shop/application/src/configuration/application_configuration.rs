use common::types::base::{AM, AMW};
use lazy_static::lazy_static;

use crate::event::kafka_event_publisher_impl::KafkaEventPublisherImpl;

// lazy_static! {
//     pub static ref EVENT_PUBLISHER: AM<EventPublisherImpl<DomainEventEnum>> =
//         AMW::new(EventPublisherImpl::default()));
// }

lazy_static! {
    /// `EventPublisher` dependency injection
    pub(super) static ref EVENT_PUBLISHER: AM<KafkaEventPublisherImpl> =
        AMW::new(KafkaEventPublisherImpl::default());
}
