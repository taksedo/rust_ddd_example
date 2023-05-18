use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::meal_name::MealName;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(new, Debug)]
pub struct AddMealToMenuUseCase {
    pub meal_persister: Rc<RefCell<dyn MealPersister>>,
    pub id_generator: Rc<dyn MealIdGenerator>,
    pub meal_exists: Rc<RefCell<dyn MealAlreadyExists>>,
}

impl AddMealToMenu for AddMealToMenuUseCase {
    fn execute(&mut self, name: MealName) -> Result<MealId, AddMealToMenuUseCaseError> {
        Meal::add_meal_to_menu(
            Rc::clone(&self.id_generator),
            Rc::clone(&self.meal_exists),
            name,
        )
        .map_err(|_| AddMealToMenuUseCaseError::AlreadyExists)
        .map(|new_meal_in_menu| {
            self.meal_persister
                .borrow_mut()
                .save(new_meal_in_menu.clone());
            new_meal_in_menu.domain_entity_field.id
        })
    }
}

impl DomainEventTrait for AddMealToMenuUseCase {}
