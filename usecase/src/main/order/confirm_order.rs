use std::fmt::Debug;

use thiserror::Error;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait ConfirmOrder: Debug + Send {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), ConfirmOrderUseCaseError>;
}

#[derive(Error, Debug, PartialEq)]
pub enum ConfirmOrderUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
