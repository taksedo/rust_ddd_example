use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use derive_new::new;

use common::types::main::base::domain_event::DomainEventTrait;
use common::types::main::errors::error::ToError;
use domain;
use domain::main::menu::meal::{Meal, MealError};
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;

use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};

#[derive(new, Debug)]
pub struct AddMealToMenuUseCase {
    pub meal_persister: Arc<Mutex<dyn MealPersister>>,
    pub id_generator: Arc<Mutex<dyn MealIdGenerator>>,
    pub meal_exists: Arc<Mutex<dyn MealAlreadyExists>>,
}

impl AddMealToMenu for AddMealToMenuUseCase {
    fn execute(
        &mut self,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<MealId, AddMealToMenuUseCaseError> {
        Meal::add_meal_to_menu(
            Arc::clone(&self.id_generator),
            Arc::clone(&self.meal_exists),
            name,
            description,
            price,
        )
        .map_err(MealError::to_error)
        .map(|new_meal_in_menu| {
            self.meal_persister
                .lock()
                .unwrap()
                .save(new_meal_in_menu.clone());
            new_meal_in_menu.entity_params.id
        })
    }
}

impl ToError<AddMealToMenuUseCaseError> for MealError {
    fn to_error(self) -> AddMealToMenuUseCaseError {
        match self {
            MealError::AlreadyExistsWithSameNameError => AddMealToMenuUseCaseError::AlreadyExists,
            _ => AddMealToMenuUseCaseError::UnknownError,
        }
    }
}

impl DomainEventTrait for AddMealToMenuUseCase {}
