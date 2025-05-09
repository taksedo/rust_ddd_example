use std::{env, net::IpAddr};

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::{Uri, header},
    middleware::Logger,
};
use log::info;
use rest::{
    menu::{
        add_meal_to_menu_endpoint::add_meal_to_menu_endpoint_config,
        get_health_status::get_health_status_config,
        get_meal_by_id_endpoint::get_meal_by_id_endpoint_config,
        get_menu_endpoint::get_menu_endpoint_config,
        remove_meal_from_menu_endpoint::remove_meal_from_menu_endpoint_config,
    },
    order::{
        cancel_order_endpoint::cancel_order_endpoint_config,
        confirm_order_endpoint::confirm_order_endpoint_config,
        get_order_by_id_endpoint::get_order_by_id_endpoint_config,
        get_orders_endpoint::get_orders_endpoint_config,
    },
};
use serde::{Deserialize, Serialize};
use tokio::{task, task::JoinHandle};
use usecase::{
    menu::scenario::{
        AddMealToMenuUseCase, GetMealByIdUseCase, GetMenuUseCase, RemoveMealFromMenuUseCase,
    },
    order::scenarios::{
        CancelOrderUseCase, ConfirmOrderUseCase, GetOrderByIdUseCase, GetOrdersUseCase,
    },
};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::configuration::{
    persistence_configuration::ORepository,
    swagger_configuration::ApiDoc,
    use_case_configuration::{
        ADD_MEAL_TO_MENU_USE_CASE, CANCEL_ORDER_USECASE, CONFIRM_ORDER_USECASE,
        GET_MEAL_BY_ID_USE_CASE, GET_MENU_USE_CASE, GET_ORDER_BY_ID, GET_ORDERS_USECASE,
        REMOVE_MEAL_FROM_MENU_USECASE,
    },
};

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenClaims {
    id: i32,
}

pub(crate) fn web_api_backend_startup() -> JoinHandle<()> {
    task::spawn(async {
        let http_host_url = env::var("HTTP_HOST_URL").unwrap();
        info!("Starting HTTP server at {}", http_host_url);

        let http_host_url = env::var("HTTP_HOST_URL").unwrap();
        let host_url = http_host_url.parse::<Uri>().unwrap();
        let host_address = host_url.host().unwrap();
        let host_port = host_url.port().unwrap();

        let openapi = ApiDoc::openapi();

        HttpServer::new(move || {
            App::new()
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", openapi.clone()),
                )
                .configure(get_health_status_config)
                .configure(add_meal_to_menu_endpoint_config::<AddMealToMenuUseCase>)
                .configure(get_meal_by_id_endpoint_config::<GetMealByIdUseCase>)
                .configure(get_menu_endpoint_config::<GetMenuUseCase>)
                .configure(remove_meal_from_menu_endpoint_config::<RemoveMealFromMenuUseCase>)
                .configure(
                    cancel_order_endpoint_config::<CancelOrderUseCase<ORepository, ORepository>>,
                )
                .configure(
                    confirm_order_endpoint_config::<ConfirmOrderUseCase<ORepository, ORepository>>,
                )
                .configure(get_order_by_id_endpoint_config::<GetOrderByIdUseCase<ORepository>>)
                .configure(get_orders_endpoint_config::<GetOrdersUseCase<ORepository>>)
                .app_data(ADD_MEAL_TO_MENU_USE_CASE.clone())
                .app_data(GET_MEAL_BY_ID_USE_CASE.clone())
                .app_data(GET_MENU_USE_CASE.clone())
                .app_data(REMOVE_MEAL_FROM_MENU_USECASE.clone())
                .app_data(CANCEL_ORDER_USECASE.clone())
                .app_data(CONFIRM_ORDER_USECASE.clone())
                .app_data(GET_ORDER_BY_ID.clone())
                .app_data(GET_ORDERS_USECASE.clone())
                .wrap(
                    Cors::default()
                        .allowed_origin(&http_host_url)
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
        })
        .bind((
            host_address
                .parse::<IpAddr>()
                .expect("Wrong IP address configured"),
            host_port
                .to_string()
                .parse::<u16>()
                .expect("Wrong port configured"),
        ))
        .unwrap()
        .run()
        .await
        .unwrap();
    })
}
