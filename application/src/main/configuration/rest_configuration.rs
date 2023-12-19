use std::{env, net::IpAddr};

use actix_cors::Cors;
use actix_web::{
    http::{header, Uri},
    middleware::Logger,
    web, App, HttpServer,
};
use dotenvy::dotenv;
use rest::main::{
    endpoint_url::{
        API_V1_MENU_ADD_TO_MENU, API_V1_MENU_DELETE_BY_ID, API_V1_MENU_GET_ALL,
        API_V1_MENU_GET_BY_ID, API_V1_ORDER_CANCEL_BY_ID, API_V1_ORDER_CONFIRM_BY_ID,
        API_V1_ORDER_GET_ALL,
    },
    menu::{
        add_meal_to_menu_endpoint, get_health_status, get_meal_by_id_endpoint, get_menu_endpoint,
        remove_meal_from_menu_endpoint,
    },
    order::{cancel_order_endpoint, confirm_order_endpoint, get_orders_endpoint},
};
use usecase::main::{
    menu::scenario::{
        add_meal_to_menu_use_case::AddMealToMenuUseCase,
        get_meal_by_id_use_case::GetMealByIdUseCase, get_menu_use_case::GetMenuUseCase,
        remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase,
    },
    order::scenarios::{
        cancel_order_use_case::CancelOrderUseCase, confirm_order_use_case::ConfirmOrderUseCase,
        get_orders_use_case::GetOrdersUseCase,
    },
};

use crate::main::configuration::use_case_configuration::{
    ADD_MEAL_TO_MENU_USE_CASE, CANCEL_ORDER_USECASE, CONFIRM_ORDER_USECASE,
    GET_MEAL_BY_ID_USE_CASE, GET_MENU_USE_CASE, GET_ORDERS_USECASE, REMOVE_MEAL_FROM_MENU_USECASE,
};

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

    log::info!("API_V1_ORDER_GET_ALL = {API_V1_ORDER_GET_ALL}");

    HttpServer::new(move || {
        App::new()
            .app_data(ADD_MEAL_TO_MENU_USE_CASE.clone())
            .app_data(GET_MEAL_BY_ID_USE_CASE.clone())
            .app_data(GET_MENU_USE_CASE.clone())
            .app_data(REMOVE_MEAL_FROM_MENU_USECASE.clone())
            .app_data(CANCEL_ORDER_USECASE.clone())
            .app_data(CONFIRM_ORDER_USECASE.clone())
            .app_data(GET_ORDERS_USECASE.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&env::var("HOST_URL").unwrap())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::LOCATION,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .route(
                API_V1_MENU_ADD_TO_MENU,
                web::post().to(add_meal_to_menu_endpoint::execute::<AddMealToMenuUseCase>),
            )
            .route(
                API_V1_MENU_GET_BY_ID,
                web::get().to(get_meal_by_id_endpoint::execute::<GetMealByIdUseCase>),
            )
            .route(
                API_V1_MENU_GET_ALL,
                web::get().to(get_menu_endpoint::execute::<GetMenuUseCase>),
            )
            .route(
                API_V1_MENU_DELETE_BY_ID,
                web::delete()
                    .to(remove_meal_from_menu_endpoint::execute::<RemoveMealFromMenuUseCase>),
            )
            .route(
                API_V1_ORDER_CANCEL_BY_ID,
                web::put().to(cancel_order_endpoint::execute::<CancelOrderUseCase>),
            )
            .route(
                API_V1_ORDER_CONFIRM_BY_ID,
                web::put().to(confirm_order_endpoint::execute::<ConfirmOrderUseCase>),
            )
            .route(
                API_V1_ORDER_GET_ALL,
                web::get().to(get_orders_endpoint::execute::<GetOrdersUseCase>),
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
