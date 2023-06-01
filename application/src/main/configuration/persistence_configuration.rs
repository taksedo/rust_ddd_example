use crate::main::configuration::application_configuration::EVENT_PUBLISHER;
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator as IdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository as MealRepository;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref MEAL_ID_GENERATOR: Arc<Mutex<IdGenerator>> = meal_id_generator();
    pub static ref MEAL_RESPOSITORY: Arc<Mutex<MealRepository>> = meal_repository();
}

pub fn meal_id_generator() -> Arc<Mutex<IdGenerator>> {
    Arc::new(Mutex::new(IdGenerator::new()))
}

pub fn meal_repository() -> Arc<Mutex<MealRepository>> {
    Arc::new(Mutex::new(MealRepository::new(
        Arc::clone(&EVENT_PUBLISHER) as _,
    )))
}
