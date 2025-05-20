use std::fmt::Debug;

use async_trait::async_trait;
use domain::{cart::value_objects::customer_id::CustomerId, order::shop_order::OrderState};
use thiserror::Error;

#[async_trait]
pub trait GetLastOrderState: Debug + Send {
    async fn execute(
        &self,
        for_customer: &CustomerId,
    ) -> Result<OrderState, GetLastOrderStateUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GetLastOrderStateUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
}
