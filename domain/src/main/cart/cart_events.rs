use common::types::main::base::domain_event::DomainEvent;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::main::cart::value_objects::cart_id::CartId;
use crate::main::menu::value_objects::meal_id::MealId;

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct CartCreatedDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub cart_id: CartId,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct MealAddedToCartDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub cart_id: CartId,
    pub meal_id: MealId,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct MealRemovedFromCartDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub cart_id: CartId,
    pub meal_id: MealId,
}

#[enum_dispatch]
trait DomainEventTrait {}

impl DomainEventTrait for CartCreatedDomainEvent {}

impl DomainEventTrait for MealAddedToCartDomainEvent {}

impl DomainEventTrait for MealRemovedFromCartDomainEvent {}

#[enum_dispatch(DomainEventTrait)]
#[derive(PartialEq, Debug, Clone, SmartDefault, Serialize, Deserialize, Hash, Eq)]
pub enum CartEventEnum {
    #[default]
    CartCreatedDomainEvent,
    MealAddedToCartDomainEvent,
    MealRemovedFromCartDomainEvent,
}
