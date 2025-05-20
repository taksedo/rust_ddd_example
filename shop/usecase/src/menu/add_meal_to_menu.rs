use std::fmt::Debug;

use async_trait::async_trait;
use domain::menu::{
    meal::MealError,
    value_objects::{
        meal_description::MealDescription, meal_id::MealId, meal_name::MealName, price::Price,
    },
};

#[async_trait]
pub trait AddMealToMenu: Debug + Send {
    async fn execute(
        &mut self,
        name: &MealName,
        description: &MealDescription,
        price: &Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddMealToMenuUseCaseError {
    InvalidParameters,
    AlreadyExists,
    UnknownError,
}

impl From<MealError> for AddMealToMenuUseCaseError {
    fn from(value: MealError) -> Self {
        match value {
            MealError::AlreadyExistsWithSameNameError => Self::AlreadyExists,
        }
    }
}
