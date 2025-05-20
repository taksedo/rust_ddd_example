use async_trait::async_trait;
use domain::menu::value_objects::meal_id::MealId;

#[async_trait]
pub trait RemoveMealFromMenu {
    async fn execute(&mut self, id: &MealId) -> Result<(), RemoveMealFromMenuUseCaseError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RemoveMealFromMenuUseCaseError {
    MealNotFound,
}
