// use std::cell::RefCell;
// use std::rc::Rc;
// use crate::main::endpoint_url::{API_V1, MENU, MENU_ADD_TO_MENU};
// use crate::main::menu::add_meal_to_meny_endpoint::{echo, hello, manual_hello};
// use actix_cors::Cors;
//
// use actix_web::http::header;
// use actix_web::middleware::Logger;
// use actix_web::{web, App, HttpServer};
// use domain::main::menu::meal::MealError::IdGenerationError;
// use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
// use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
// use menu::add_meal_to_meny_endpoint::*;
// use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
//
// pub mod endpoint_url;
// pub mod menu;
//
// #[actix_web::main]
// pub async fn start_web_backend() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
//     type I = InMemoryIncrementalMealIdGenerator;
//     type MP = InMemoryMealRepository<P, E>;
//
//
//     let meal_id_generator = InMemoryIncrementalMealIdGenerator::new();
//     let event_publisher = Rc::new(RefCell::new(TestEventPublisher::new()));
//     let meal_persister = InMemoryMealRepository::new(/* event_publisher */);
//
//
//
//     let add_meal_to_menu = AddMealToMenuUseCase::<MP<P, E>, I, E>::new(meal_persister, meal_id_generator);
//
//     HttpServer::new(move || {
//         App::new()
//             .wrap(
//                 Cors::default()
//                     .allowed_origin("http://localhost:8080")
//                     .allowed_methods(vec!["GET", "POST"])
//                     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
//                     .allowed_header(header::CONTENT_TYPE)
//                     .supports_credentials()
//                     .max_age(3600),
//             )
//             .wrap(Logger::default())
//             .service(info)
//             .service(web::scope(API_V1).service(web::scope(MENU).service(add_meal_to_menu)))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
