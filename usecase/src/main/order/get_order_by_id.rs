use std::fmt::Debug;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

use crate::main::order::dto::order_details::OrderDetails;

pub trait GetOrderById: Debug + Send {
    fn execute(&mut self, id: ShopOrderId) -> Result<OrderDetails, GetOrderByIdUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq, Copy)]
pub enum GetOrderByIdUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
}
