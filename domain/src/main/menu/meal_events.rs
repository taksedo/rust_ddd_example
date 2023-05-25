use crate::main::menu::meal_id::MealId;
// #[cfg(test)]
use crate::test_fixtures::fixtures::TestEvent;
use common_types::main::base::domain_event::EventId;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

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

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct MealRemovedFromMenuDomainEvent {
    #[new(value = "EventId::new()")]
    pub id: EventId,
    pub meal_id: MealId,
    #[new(value = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
}

impl Default for MealRemovedFromMenuDomainEvent {
    fn default() -> Self {
        Self {
            id: Default::default(),
            meal_id: Default::default(),
            created: OffsetDateTime::now_utc(),
        }
    }
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
    // #[cfg(test)]
    TestEvent,
}
