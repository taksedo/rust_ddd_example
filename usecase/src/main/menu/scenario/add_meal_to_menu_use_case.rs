use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

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
        .map_err(|_| AddMealToMenuUseCaseError::AlreadyExists)
        .map(|new_meal_in_menu| {
            self.meal_persister
                .lock()
                .unwrap()
                .save(new_meal_in_menu.clone());
            new_meal_in_menu.entity_params.id
        })
    }
}

impl DomainEventTrait for AddMealToMenuUseCase {}
