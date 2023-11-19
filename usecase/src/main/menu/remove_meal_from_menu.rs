use domain::main::menu::value_objects::meal_id::MealId;

pub trait RemoveMealFromMenu {
    fn execute(&mut self, id: MealId) -> Result<(), RemoveMealFromMenuUseCaseError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RemoveMealFromMenuUseCaseError {
    MealNotFound,
}
