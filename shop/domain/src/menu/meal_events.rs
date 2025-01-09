use common::types::base::{DomainEventTrait, EventId};
use derive_new::new;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::menu::value_objects::meal_id::MealId;

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

#[enum_delegate::implement(DomainEventTrait)]
#[derive(PartialEq, Debug, Clone, SmartDefault, Serialize, Deserialize, Hash, Eq)]
pub enum MealEventEnum {
    #[default]
    MealRemovedFromMenuDomainEvent(MealRemovedFromMenuDomainEvent),
    MealAddedToMenuDomainEvent(MealAddedToMenuDomainEvent),
}
