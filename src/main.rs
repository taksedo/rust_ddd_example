#![allow(unused_imports)]

use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use common_types::main::base::domain_entity::Version;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::main::menu::meal_name::MealName;
// use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
// use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
// use in_memory_persistence::main::menu::*;
// use rest::main::start_web_backend;
use std::env;
use std::rc::Rc;

// use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuRequest};
// use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
// use usecase::test_fixtures::test_fixtures::TestMealPersister;

fn main() {
    // let mut id_generator = InMemoryIncrementalMealIdGenerator::new();
    // let mut meal_persister = TestMealPersister::new();
    // let mut usecase = AddMealToMenuUseCase::new(meal_persister, id_generator);
    // dotenv::dotenv();
    // init_logger();

    // let _backend = start_web_backend();
}

fn init_logger() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".into());
    let log_level = log_level.parse().unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = std::fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }
    Ok(builder.apply()?)
}
