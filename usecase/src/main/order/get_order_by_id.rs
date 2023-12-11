use std::fmt::Debug;

use thiserror::Error;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::dto::order_details::OrderDetails;

pub trait GetOrderById: Debug + Send {
    fn execute(&self, id: ShopOrderId) -> Result<OrderDetails, GetOrderByIdUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GetOrderByIdUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
}
