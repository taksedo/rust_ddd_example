use std::fmt::Debug;

use thiserror::Error;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::meal_id::MealId;

pub trait RemoveMealFromCart: Debug + Send {
    fn execute(
        &self,
        for_customer: CustomerId,
        meal_id: MealId,
    ) -> Result<(), RemoveMealFromCartUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RemoveMealFromCartUseCaseError {
    #[error("Cart not found")]
    CartNotFound,
}
