use crate::main::endpoint_url::{API_V1, MENU, MENU_ADD_TO_MENU};
use crate::main::menu::add_meal_to_menu_endpoint::{echo, hello, manual_hello};
use actix_cors::Cors;
use std::cell::RefCell;
use std::rc::Rc;

use crate::main::menu::add_meal_to_menu_endpoint;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use actix_web::{web, App, HttpServer};
use application::main::event::event_publisher_impl::EventPublisherImpl;
use domain::main::menu::meal::MealError::IdGenerationError;
use domain::main::menu::meal_events::DomainEventEnum;
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use in_memory_persistence::test_fixtures::fixtures::TestEventPublisher;
use menu::add_meal_to_menu_endpoint::*;
use usecase::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;

pub mod endpoint_url;
pub mod menu;

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let meal_id_generator = Rc::new(RefCell::new(InMemoryIncrementalMealIdGenerator::new()));
    let meal_publisher = EventPublisherImpl::<DomainEventEnum>::default();
    let meal_repository = InMemoryMealRepository::new(Rc::new(RefCell::new((meal_publisher))));

    let meal_repository_ref = Rc::new(RefCell::new(meal_repository));

    let rule = MealAlreadyExistsUsesMealExtractor::new(Rc::clone(&meal_repository_ref) as _);
    let mut add_meal_to_menu = AddMealToMenuUseCase::new(
        Rc::clone(&meal_repository_ref) as _,
        meal_id_generator,
        Rc::new(RefCell::new(rule)),
    );

    let mut counter = web::Data::new(add_meal_to_menu);

    HttpServer::new(move || {
        App::new()
            // .app_data(counter.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(info)
            .service(web::scope(API_V1).service(
                web::scope(MENU).route("/add", web::post().to(add_meal_to_menu_endpoint::execute)),
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
