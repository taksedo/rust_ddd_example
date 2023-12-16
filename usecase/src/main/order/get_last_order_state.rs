use std::fmt::Debug;

use domain::main::{cart::value_objects::customer_id::CustomerId, order::shop_order::OrderState};
use thiserror::Error;

pub trait GetLastOrderState: Debug + Send {
    fn execute(
        &self,
        for_customer: CustomerId,
    ) -> Result<OrderState, GetLastOrderStateUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GetLastOrderStateUseCaseError {
    #[error("Order not found")]
    OrderNotFound,
}
