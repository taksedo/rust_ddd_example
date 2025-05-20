use std::fmt::Debug;

use async_trait::async_trait;
use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

#[async_trait]
pub trait CancelOrder: Debug + Send {
    async fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), CancelOrderUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq, Copy)]
pub enum CancelOrderUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
