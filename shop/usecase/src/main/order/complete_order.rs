use std::fmt::Debug;

use domain::order::value_objects::shop_order_id::ShopOrderId;

pub trait CompleteOrder: Debug + Send {
    fn execute(&self, order_id: &ShopOrderId) -> Result<(), CompleteOrderUseCaseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompleteOrderUseCaseError {
    OrderNotFound,
    InvalidOrderState,
}
