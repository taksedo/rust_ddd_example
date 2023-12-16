use std::fmt::Debug;

use domain::main::{
    cart::value_objects::customer_id::CustomerId, menu::value_objects::meal_id::MealId,
};
use thiserror::Error;

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
