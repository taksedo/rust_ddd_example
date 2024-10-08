use std::fmt::Debug;

use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

pub trait ConfirmOrder: Debug + Send {
    fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), ConfirmOrderUseCaseError>;
}

#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum ConfirmOrderUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
