use std::fmt::Debug;

use derive_new::new;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;
use uuid::Uuid;

/// Abstract struct for `DomainEvent`
#[derive(
    Debug, Clone, PartialEq, SmartDefault, Serialize, Deserialize, Hash, Eq, Ord, PartialOrd,
)]
pub struct DomainEvent {
    #[default(EventId::new())]
    pub id: EventId,
    #[default(OffsetDateTime::now_utc())]
    created: OffsetDateTime,
}

#[derive(
    new, PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, Hash, Ord, PartialOrd,
)]
pub struct EventId {
    #[new(value = "Uuid::new_v4()")]
    pub(crate) value: Uuid,
}

#[enum_delegate::register]
pub trait DomainEventTrait {}

// todo возможно понадобится
// serialize_trait_object!(DomainEventTrait<T>);

// impl DomainEventTrait for DomainEvent {}

// impl dyn DomainEventTrait + 'static {
//     pub fn downcast_ref<T: DomainEventTrait + 'static>(&self) -> Option<&T> {
//         unsafe { Some(&*(self as *const dyn DomainEventTrait as *const T)) }
//     }
// }

#[cfg(test)]
mod test {
    use derive_new::new;

    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn create_event__check_event_id_is_unique() {
        let firstEvent = EmptyEvent::new();
        let secondEvent = EmptyEvent::new();
        assert_ne!(
            firstEvent.domain_events_params.id,
            secondEvent.domain_events_params.id
        );
        assert_ne!(
            firstEvent.domain_events_params.id.value,
            secondEvent.domain_events_params.id.value
        )
    }

    #[derive(new)]
    struct EmptyEvent {
        #[new(value = "DomainEvent::default()")]
        domain_events_params: DomainEvent,
    }
}
