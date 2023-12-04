use thiserror::Error;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::order::shop_order::OrderState;

pub trait GetLastOrderState {
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
