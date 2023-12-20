use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{resource_not_found, to_invalid_param_bad_request};
use domain::main::menu::value_objects::meal_id::MealId;
use usecase::main::menu::remove_meal_from_menu::{
    RemoveMealFromMenu, RemoveMealFromMenuUseCaseError,
};

use crate::main::{to_error::ToRestError, validated::Validated};

pub async fn execute<T: RemoveMealFromMenu + Send + Debug>(
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
