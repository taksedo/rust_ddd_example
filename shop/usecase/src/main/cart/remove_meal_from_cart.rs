use std::fmt::Debug;

use domain::{cart::value_objects::customer_id::CustomerId, menu::value_objects::meal_id::MealId};
use thiserror::Error;

pub trait RemoveMealFromCart: Debug + Send {
    fn execute(
        &self,
        for_customer: &CustomerId,
        meal_id: &MealId,
    ) -> Result<(), RemoveMealFromCartUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RemoveMealFromCartUseCaseError {
    #[error("Cart not found")]
    CartNotFound,
}
