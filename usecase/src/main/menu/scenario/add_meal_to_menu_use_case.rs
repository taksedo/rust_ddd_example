use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::{
    AddMealToMenu, AddMealToMenuRequest, AddMealToMenuUseCaseError,
};
use derive_new::new;
use domain;
use domain::main::menu::meal::{Meal, MealError};
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::rc::Rc;

#[derive(new, Debug)]
pub struct AddMealToMenuUseCase {
    pub meal_persister: Box<dyn MealPersister>,
    pub id_generator: Rc<dyn MealIdGenerator>,
    // meal_exists: Rc<dyn MealAlreadyExists>,
}

impl AddMealToMenu for AddMealToMenuUseCase {
    fn execute(
        &mut self,
        request: AddMealToMenuRequest,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        Meal::add_meal_to_menu(Rc::clone(&self.id_generator), request.name)
            .map_err(|_| AddMealToMenuUseCaseError::AlreadyExists)
            .map(|new_meal_in_menu| {
                self.meal_persister.save(new_meal_in_menu.to_owned());
                new_meal_in_menu.id
            })
    }
}
