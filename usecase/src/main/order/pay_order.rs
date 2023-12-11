use std::fmt::Debug;

use derive_new::new;
use thiserror::Error;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait PayOrder: Debug + Send {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), PayOrderHandlerError>;
}

#[derive(new, Error, Debug, Clone, PartialEq)]
pub enum PayOrderHandlerError {
    #[error("Order not found")]
    OrderNotFound,
    #[error("Invalid order state")]
    InvalidOrderState,
}
