use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use common::types::main::{base::domain_event::DomainEventTrait, errors::error::ToError};
use derive_new::new;
use domain::{
    self,
    main::menu::{
        meal::{Meal, MealError},
        meal_already_exists::MealAlreadyExists,
        value_objects::{
            meal_description::MealDescription,
            meal_id::{MealId, MealIdGenerator},
            meal_name::MealName,
            price::Price,
        },
    },
};

use crate::main::menu::{
    access::meal_persister::MealPersister,
    add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError},
};

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
            self.id_generator.clone(),
            self.meal_exists.clone(),
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

#[allow(unreachable_patterns)]
impl ToError<AddMealToMenuUseCaseError> for MealError {
    fn to_error(self) -> AddMealToMenuUseCaseError {
        match self {
            MealError::AlreadyExistsWithSameNameError => AddMealToMenuUseCaseError::AlreadyExists,
            _ => AddMealToMenuUseCaseError::UnknownError,
        }
    }
}

impl DomainEventTrait for AddMealToMenuUseCase {}
