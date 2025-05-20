use std::fmt::Debug;

use async_trait::async_trait;
use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

use crate::order::dto::order_details::OrderDetails;

#[async_trait]
pub trait GetOrderById: Debug + Send {
    async fn execute(&mut self, id: &ShopOrderId)
    -> Result<OrderDetails, GetOrderByIdUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq, Copy)]
pub enum GetOrderByIdUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
}
