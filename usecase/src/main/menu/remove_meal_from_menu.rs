use domain::main::menu::meal_id::MealId;

pub trait RemoveMealFromMenu {
    fn execute(&mut self, id: MealId) -> Result<(), RemoveMealFromMenuUseCaseError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Clone, Copy)]
pub enum RemoveMealFromMenuUseCaseError {
    #[error("Еда не найдена")]
    MealNotFound,
}
