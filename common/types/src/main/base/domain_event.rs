use derive_new::new;
use std::fmt::{Debug, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(new, Debug, Clone)]
pub struct DomainEvent<E: DomainEventTrait> {
    #[new(value = "EventId::new()")]
    id: EventId,
    events: Vec<E>,
}

#[derive(new, PartialEq, Eq, Debug, Clone)]
pub struct EventId {
    #[new(value = "Uuid::new_v4()")]
    value: Uuid,
    #[new(value = "OffsetDateTime::now_utc()")]
    created: OffsetDateTime,
}

pub trait DomainEventTrait: Debug {}
// todo возможно понадобится
// serialize_trait_object!(DomainEventTrait<T>);

impl<E: DomainEventTrait> DomainEventTrait for DomainEvent<E> {}
