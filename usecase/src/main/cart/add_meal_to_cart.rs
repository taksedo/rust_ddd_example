use std::fmt::Debug;

use thiserror::Error;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::meal_id::MealId;

pub trait AddMealToCart: Debug + Send {
    fn execute(
        &mut self,
        for_customer: CustomerId,
        meal_id: MealId,
    ) -> Result<(), AddMealToCartUseCaseError>;
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AddMealToCartUseCaseError {
    #[error("Meal Not Found")]
    MealNotFound,
}
