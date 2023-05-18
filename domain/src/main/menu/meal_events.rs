use crate::main::menu::meal_id::MealId;
use common_types::main::base::domain_event::{DomainEventTrait, EventId};
use derive_new::new;
use time::OffsetDateTime;

#[derive(new, Debug, Clone)]
pub struct MealAddedToMenuDomainEvent {
    #[new(value = "EventId::new()")]
    pub id: EventId,
    pub meal_id: MealId,
    #[new(value = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
}

#[derive(new, Debug, Clone)]
pub struct MealRemovedFromMenuDomainEvent {
    #[new(value = "EventId::new()")]
    pub id: EventId,
    pub meal_id: MealId,
    #[new(value = "OffsetDateTime::now_utc()")]
    pub created: OffsetDateTime,
}

impl DomainEventTrait for MealAddedToMenuDomainEvent {}

impl DomainEventTrait for MealRemovedFromMenuDomainEvent {}
