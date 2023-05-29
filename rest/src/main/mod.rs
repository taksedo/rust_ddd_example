use crate::main::endpoint_url::{MENU_ADD_TO_MENU, MENU_GET_ALL, MENU_GET_BY_ID};
use crate::main::menu::add_meal_to_menu_endpoint::AddMealToMenuEndpointSharedState;
use crate::main::menu::get_meal_by_id_endpoint::GetMealByIdEndpointSharedState;
use crate::main::menu::get_menu_endpoint::GetMenuEndpointSharedState;
use crate::main::menu::shared_state::{
    mea_get_menu_shared_state, meal_create_repository, meal_create_shared_state,
    meal_get_by_id_shared_state,
};
use crate::main::menu::{add_meal_to_menu_endpoint, get_meal_by_id_endpoint, get_menu_endpoint};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use std::sync::{Arc, Mutex};
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use usecase::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;
use usecase::main::menu::scenario::get_menu_use_case::GetMenuUseCase;

pub mod endpoint_url;
pub mod menu;

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    let id_generator = Arc::new(Mutex::new(InMemoryIncrementalMealIdGenerator::new()));
    let meal_repository = meal_create_repository();

    let add_meal_to_menu_shared_state =
        meal_create_shared_state::<InMemoryMealRepository, InMemoryIncrementalMealIdGenerator>(
            Arc::clone(&meal_repository) as _,
            Arc::clone(&id_generator) as _,
        );
    let meal_add_counter = web::Data::new(AddMealToMenuEndpointSharedState::new(
        add_meal_to_menu_shared_state,
    ));

    let get_meal_by_id_shared_state = meal_get_by_id_shared_state(Arc::clone(&meal_repository));
    let meal_get_by_id_counter = web::Data::new(GetMealByIdEndpointSharedState::new(
        get_meal_by_id_shared_state,
    ));

    let get_menu_shared_state = mea_get_menu_shared_state(Arc::clone(&meal_repository));
    let meal_get_menu_counter =
        web::Data::new(GetMenuEndpointSharedState::new(get_menu_shared_state));

    HttpServer::new(move || {
        App::new()
            .app_data(meal_add_counter.clone())
            .app_data(meal_get_by_id_counter.clone())
            .app_data(meal_get_menu_counter.clone())
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
            .route(
                MENU_ADD_TO_MENU,
                web::post().to(add_meal_to_menu_endpoint::execute::<AddMealToMenuUseCase>),
            )
            .route(
                MENU_GET_BY_ID,
                web::get().to(get_meal_by_id_endpoint::execute::<GetMealByIdUseCase>),
            )
            .route(
                MENU_GET_ALL,
                web::get().to(get_menu_endpoint::execute::<GetMenuUseCase>),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
