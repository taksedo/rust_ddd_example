use thiserror::Error;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait CancelOrder {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), CancelOrderUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CancelOrderUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
