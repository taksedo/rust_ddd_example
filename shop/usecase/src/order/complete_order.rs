use std::fmt::Debug;

use async_trait::async_trait;
use domain::order::value_objects::shop_order_id::ShopOrderId;

#[async_trait]
pub trait CompleteOrder: Debug + Send {
    async fn execute(&self, order_id: &ShopOrderId) -> Result<(), CompleteOrderUseCaseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompleteOrderUseCaseError {
    OrderNotFound,
    InvalidOrderState,
}
