#![allow(unused_imports)]

use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use common_types::main::base::domain_entity::Version;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::meal_name::MealName;
use rest::main::start_web_backend;
use std::env;
use std::rc::Rc;

fn main() {
    // let mut id_generator = InMemoryIncrementalMealIdGenerator::new();
    // let mut meal_persister = TestMealPersister::new();
    // let mut usecase = AddMealToMenuUseCase::new(meal_persister, id_generator);
    // dotenv::dotenv();
    // init_logger();

    let _backend = start_web_backend();
}
