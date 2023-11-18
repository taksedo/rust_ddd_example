use actix_web::ResponseError;
use domain::main::menu::value_objects::meal_id::MealId;

pub trait RemoveMealFromMenu {
    fn execute(&mut self, id: MealId) -> Result<(), RemoveMealFromMenuUseCaseError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Clone, Copy)]
pub enum RemoveMealFromMenuUseCaseError {
    #[error("")]
    MealNotFound,
}

impl ResponseError for RemoveMealFromMenuUseCaseError {}
