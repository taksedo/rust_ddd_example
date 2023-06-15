use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::fmt::Debug;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(new, Debug)]
pub struct DomainEvent {
    #[new(value = "EventId::new()")]
    pub id: EventId,
    #[new(value = "OffsetDateTime::now_utc()")]
    created: OffsetDateTime,
}

#[derive(new, PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct EventId {
    #[new(value = "Uuid::new_v4()")]
    pub(crate) value: Uuid,
}

#[enum_dispatch]
pub trait DomainEventTrait: Debug {}

impl DomainEventTrait for DomainEvent {}

#[derive(SmartDefault, Debug)]
pub struct DomainEventNew<EventType: DomainEventType> {
    #[default(_code = "EventId::new()")]
    pub id: EventId,
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
    pub event_type: EventType,
}

impl<EventType: DomainEventType> DomainEventNew<EventType> {
    pub fn new() -> DomainEventBuilder<EventType> {
        DomainEventBuilder::default()
    }
}

pub trait ReturnEventType {
    fn return_event_type() -> String;
}

impl<EventType: DomainEventType> ReturnEventType for DomainEventNew<EventType> {
    fn return_event_type() -> String {
        format!("EventType:?")
    }
}

pub trait DomainEventType: Default + Debug {}

pub trait NewDomainEventType<T>: Sized {
    fn new(id: T) -> Self;
}

#[derive(SmartDefault, new)]
pub struct DomainEventBuilder<EventType: DomainEventType> {
    #[default(_code = "EventId::new()")]
    pub id: EventId,
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
    pub event_type: EventType,
}

impl<EventType: DomainEventType + From<i64> + Clone> DomainEventBuilder<EventType> {
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.event_type = EventType::from(id);
        self
    }
    pub fn build(&mut self) -> DomainEventNew<EventType> {
        DomainEventNew {
            id: EventId::new(),
            created: OffsetDateTime::now_utc(),
            event_type: self.event_type.clone(),
        }
    }
}
