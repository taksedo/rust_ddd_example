use common::types::base::domain_event::EventId;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::menu::value_objects::meal_id::MealId;
#[cfg(test)]
use crate::test_fixtures::TestEvent;

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

impl MealEventTrait for MealAddedToMenuDomainEvent {}

impl MealEventTrait for MealRemovedFromMenuDomainEvent {}

#[enum_dispatch(MealEventTrait)]
#[derive(PartialEq, Debug, Clone, SmartDefault, Serialize, Deserialize, Hash, Eq)]
pub enum MealEventEnum {
    #[default]
    MealRemovedFromMenuDomainEvent,
    MealAddedToMenuDomainEvent,
    #[cfg(test)]
    TestEvent,
}

#[allow(dead_code)]
#[enum_dispatch]
trait MealEventTrait {}
