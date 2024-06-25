use common::types::base::domain_event::DomainEvent;
use derive_new::new;
use enum_dispatch::enum_dispatch;
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

impl ShopOrderEventTrait for ShopOrderCreatedDomainEvent {}

impl ShopOrderEventTrait for ShopOrderCompletedDomainEvent {}

impl ShopOrderEventTrait for ShopOrderConfirmedDomainEvent {}

impl ShopOrderEventTrait for ShopOrderCancelledDomainEvent {}

impl ShopOrderEventTrait for ShopOrderPaidDomainEvent {}

#[enum_dispatch(ShopOrderEventTrait)]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Hash, Eq, SmartDefault)]
pub enum ShopOrderEventEnum {
    #[default]
    ShopOrderCreatedDomainEvent,
    ShopOrderCompletedDomainEvent,
    ShopOrderConfirmedDomainEvent,
    ShopOrderCancelledDomainEvent,
    ShopOrderPaidDomainEvent,
}

#[allow(dead_code)]
#[enum_dispatch]
trait ShopOrderEventTrait {}
