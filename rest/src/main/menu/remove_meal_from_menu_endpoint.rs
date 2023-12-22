use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{resource_not_found, to_invalid_param_bad_request};
use domain::main::menu::value_objects::meal_id::MealId;
use usecase::main::menu::{
    remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
    scenario::remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase,
};

use crate::main::{
    endpoint_url::API_V1_MENU_DELETE_BY_ID, to_error::ToRestError, validated::Validated,
};

pub async fn remove_meal_from_menu_endpoint<T: RemoveMealFromMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match MealId::validated(id, error_list.clone()) {
        Ok(meal_id) => match shared_state.lock().unwrap().execute(meal_id) {
            Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for RemoveMealFromMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}

pub fn remove_meal_from_menu_endpoint_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        API_V1_MENU_DELETE_BY_ID,
        web::delete().to(remove_meal_from_menu_endpoint::<RemoveMealFromMenuUseCase>),
    );
}
