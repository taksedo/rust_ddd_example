use crate::main::menu::access::meal_persister::MealPersister;
use crate::main::menu::add_meal_to_menu::AddMealToMenuUseCaseError;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_description::MealDescription;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::meal_name::MealName;
use domain::main::menu::price::Price;
use std::sync::{Arc, Mutex};

pub fn add_meal_to_menu_use_case(
    meal_persister: Arc<Mutex<impl MealPersister>>,
    id_generator: Arc<Mutex<impl MealIdGenerator>>,
    meal_exists: Arc<Mutex<impl MealAlreadyExists>>,
    name: MealName,
    description: MealDescription,
    price: Price,
) -> Result<MealId, AddMealToMenuUseCaseError> {
    Meal::add_meal_to_menu_fn(
        Arc::clone(&id_generator),
        Arc::clone(&meal_exists),
        name,
        description,
        price,
    )
    .map_err(|_| AddMealToMenuUseCaseError::AlreadyExists)
    .map(|new_meal_in_menu| {
        meal_persister
            .lock()
            .unwrap()
            .save(new_meal_in_menu.clone());
        new_meal_in_menu.domain_entity_field.id
    })
}
