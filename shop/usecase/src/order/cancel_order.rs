use std::fmt::Debug;

use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

pub trait CancelOrder: Debug + Send {
    fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), CancelOrderUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq, Copy)]
pub enum CancelOrderUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
