use crate::main::endpoint_url::{MENU_ADD_TO_MENU, MENU_GET_BY_ID};
use crate::main::menu::add_meal_to_menu_endpoint;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use application::main::event::event_publisher_impl::EventPublisherImpl;
use domain::main::menu::meal_events::DomainEventEnum;
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use menu::add_meal_to_menu_endpoint::*;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::add_meal_to_menu::AddMealToMenu;
use usecase::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;

pub mod endpoint_url;
pub mod menu;

pub fn meal_create_shared_state<T: Debug + Send + AddMealToMenu>(
) -> Arc<Mutex<AddMealToMenuUseCase>> {
    let meal_id_generator = Arc::new(Mutex::new(InMemoryIncrementalMealIdGenerator::new()));
    let meal_publisher = EventPublisherImpl::<DomainEventEnum>::default();
    let meal_repository = InMemoryMealRepository::new(Arc::new(Mutex::new(meal_publisher)));
    let meal_repository_ref = Arc::new(Mutex::new(meal_repository));
    let rule = MealAlreadyExistsUsesMealExtractor::new(Arc::clone(&meal_repository_ref) as _);

    let usecase = AddMealToMenuUseCase::new(
        Arc::clone(&meal_repository_ref) as _,
        meal_id_generator,
        Arc::new(Mutex::new(rule)),
    );
    Arc::new(Mutex::new(usecase))
}

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let add_meal_to_menu_shared_state = meal_create_shared_state::<AddMealToMenuUseCase>();

    let counter = web::Data::new(AddMealToMenuEndPointSharedState::new(
        add_meal_to_menu_shared_state,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
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
            .service(echo)
            .route(
                MENU_ADD_TO_MENU,
                web::post().to(add_meal_to_menu_endpoint::execute::<AddMealToMenuUseCase>),
            )
            .route(MENU_GET_BY_ID, web::delete().)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
