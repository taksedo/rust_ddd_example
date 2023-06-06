use crate::main::configuration::use_case_configuration::{
    ADD_MEAL_TO_MEANU_USE_CASE, GET_MEAL_BY_ID_USE_CASE, GET_MENU_USE_CASE,
    REMOVE_MEAL_FROM_MENU_USECASE,
};
use actix_cors::Cors;
use actix_web::http::{header, Uri};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use rest::main::endpoint_url::{MENU_ADD_TO_MENU, MENU_DELETE_BY_ID, MENU_GET_ALL, MENU_GET_BY_ID};
use rest::main::menu::{
    add_meal_to_menu_endpoint, get_health_status, get_meal_by_id_endpoint, get_menu_endpoint,
    remove_meal_from_menu_endpoint,
};
use std::env;
use std::net::IpAddr;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use usecase::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;
use usecase::main::menu::scenario::get_menu_use_case::GetMenuUseCase;
use usecase::main::menu::scenario::remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase;

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(env::var("LOG_LEVEL").unwrap()),
    );
    log::info!("Log level is set to {:?}", env::var("LOG_LEVEL").unwrap());

    let host_url = env::var("HOST_URL").unwrap().parse::<Uri>().unwrap();
    let host_address = host_url.host().unwrap();
    let host_port = host_url.port().unwrap();

    log::info!("starting HTTP server at {}", env::var("HOST_URL").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(ADD_MEAL_TO_MEANU_USE_CASE.clone())
            .app_data(GET_MEAL_BY_ID_USE_CASE.clone())
            .app_data(GET_MENU_USE_CASE.clone())
            .app_data(REMOVE_MEAL_FROM_MENU_USECASE.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&env::var("HOST_URL").unwrap())
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
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
            .route(
                MENU_DELETE_BY_ID,
                web::delete()
                    .to(remove_meal_from_menu_endpoint::execute::<RemoveMealFromMenuUseCase>),
            )
            .route("/health", web::get().to(get_health_status::execute))
    })
    .bind((
        host_address
            .parse::<IpAddr>()
            .expect("Wrong IP address configured"),
        host_port
            .to_string()
            .parse::<u16>()
            .expect("Wrong port configured"),
    ))?
    .run()
    .await
}
