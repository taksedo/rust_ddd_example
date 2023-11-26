use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use common::types::main::base::domain_event::EventId;

use crate::main::menu::value_objects::meal_id::MealId;
#[cfg(test)]
use crate::test_fixtures::fixtures::TestEvent;

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, SmartDefault)]
pub struct MealAddedToMenuDomainEvent {
    #[new(value = "EventId::new()")]
    #[default(Default::default())]
    pub id: EventId,
    #[default(Default::default())]
    pub meal_id: MealId,
    #[new(value = "OffsetDateTime::now_utc()")]
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, SmartDefault)]
pub struct MealRemovedFromMenuDomainEvent {
    #[new(value = "EventId::new()")]
    #[default(Default::default())]
    pub id: EventId,
    #[default(Default::default())]
    pub meal_id: MealId,
    #[new(value = "OffsetDateTime::now_utc()")]
    #[default(_code = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
}

#[enum_dispatch]
trait DomainEventTrait {}

impl DomainEventTrait for MealAddedToMenuDomainEvent {}

impl DomainEventTrait for MealRemovedFromMenuDomainEvent {}

#[enum_dispatch(DomainEventTrait)]
#[derive(PartialEq, Debug, Clone, SmartDefault, Serialize, Deserialize, Hash, Eq)]
pub enum DomainEventEnum {
    #[default]
    MealRemovedFromMenuDomainEvent,
    MealAddedToMenuDomainEvent,
    #[cfg(test)]
    TestEvent,
}
