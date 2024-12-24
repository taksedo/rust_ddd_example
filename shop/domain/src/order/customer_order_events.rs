use common::types::base::{DomainEvent, DomainEventTrait};
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::{
    cart::value_objects::customer_id::CustomerId, menu::value_objects::price::Price,
    order::value_objects::shop_order_id::ShopOrderId,
};
#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderCreatedDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub order_id: ShopOrderId,
    pub for_customer: CustomerId,
    pub total_price: Price,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderCompletedDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub order_id: ShopOrderId,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderConfirmedDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub order_id: ShopOrderId,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderCancelledDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub order_id: ShopOrderId,
}

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Default)]
pub struct ShopOrderPaidDomainEvent {
    #[new(value = "DomainEvent::default()")]
    domain_event_params: DomainEvent,
    pub order_id: ShopOrderId,
}

#[enum_delegate::implement(DomainEventTrait)]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Hash, Eq, SmartDefault)]
pub enum ShopOrderEventEnum {
    #[default]
    ShopOrderCreatedDomainEvent(ShopOrderCreatedDomainEvent),
    ShopOrderCompletedDomainEvent(ShopOrderCompletedDomainEvent),
    ShopOrderConfirmedDomainEvent(ShopOrderConfirmedDomainEvent),
    ShopOrderCancelledDomainEvent(ShopOrderCancelledDomainEvent),
    ShopOrderPaidDomainEvent(ShopOrderPaidDomainEvent),
}
