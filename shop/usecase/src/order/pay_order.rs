use std::fmt::Debug;

use async_trait::async_trait;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

#[async_trait]
pub trait PayOrder: Debug + Send {
    async fn execute(&self, order_id: &ShopOrderId) -> Result<(), PayOrderHandlerError>;
}

#[derive(new, Error, Debug, Clone, PartialEq)]
pub enum PayOrderHandlerError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
