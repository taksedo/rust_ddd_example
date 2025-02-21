use std::sync::LazyLock;

use common::types::base::{AM, ArcMutexTrait};

use crate::event::kafka_event_publisher_impl::KafkaEventPublisherImpl;

// pub static EVENT_PUBLISHER: LazyLock<AM<EventPublisherImpl<DomainEventEnum>>> =
//     LazyLock::new(|| AM::new_am(EventPublisherImpl::default()));

/// `EventPublisher` dependency injection
pub(super) static EVENT_PUBLISHER: LazyLock<AM<KafkaEventPublisherImpl>> =
    LazyLock::new(|| AM::new_am(KafkaEventPublisherImpl::default()));
