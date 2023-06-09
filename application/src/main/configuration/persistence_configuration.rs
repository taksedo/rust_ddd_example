use crate::main::configuration::application_configuration::EVENT_PUBLISHER;
use lazy_static::lazy_static;
use postgres_persistence::main::postgres_meal_id_generator::PostgresMealIdGenerator as IdGenerator;
use postgres_persistence::main::postgres_meal_repository::PostgresMealRepository as MealRepository;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref MEAL_ID_GENERATOR: Arc<Mutex<IdGenerator>> = meal_id_generator();
    pub static ref MEAL_REPOSITORY: Arc<Mutex<MealRepository>> = meal_repository();
}

pub fn meal_id_generator() -> Arc<Mutex<IdGenerator>> {
    Arc::new(Mutex::new(IdGenerator::new()))
}

pub fn meal_repository() -> Arc<Mutex<MealRepository>> {
    Arc::new(Mutex::new(MealRepository::new(
        Arc::clone(&EVENT_PUBLISHER) as _,
    )))
}
