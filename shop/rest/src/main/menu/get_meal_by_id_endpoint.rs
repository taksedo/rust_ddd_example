use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use common::common_rest::main::rest_responses::{resource_not_found, to_invalid_param_bad_request, get_json_from_http_response};
use domain::main::menu::value_objects::meal_id::MealId;
use usecase::main::menu::{
    get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
    scenario::get_meal_by_id_use_case::GetMealByIdUseCase,
};

use crate::main::{
    endpoint_url::API_V1_MENU_GET_BY_ID, menu::meal_model::MealModel, to_error::ToRestError,
    validated::Validated,
};

/// Get a meal by id
#[utoipa::path(
    get,
    path = API_V1_MENU_GET_BY_ID,
    tag = "Meal", 
    params(
        (
            "id" = i64,
            Path,
            description = "Meal id"
        )
    ),
    responses(        
        (
            status = NOT_FOUND,
            description = "Meal not found",
            body = GenericErrorResponse,
            example = json!(&(get_json_from_http_response(resource_not_found())))
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid id",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type": "http://0.0.0.0:8080/bad_request",
                    "title": "Bad request",
                    "status": 400,
                    "invalid_params": 
                    [
                        {"message": "Meal Id must be > 0"}
                    ]
                }
            )
        ),
    )
)]
pub async fn get_meal_by_id_endpoint<T: GetMealById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let error_list = Arc::new(Mutex::new(vec![]));

    match MealId::validated(id, error_list.clone()) {
        Ok(meal_id) => match shared_state.lock().unwrap().execute(meal_id) {
            Ok(meal_info) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&MealModel::from(meal_info)).unwrap()),
            Err(e) => e.to_rest_error(),
        },
        Err(_) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for GetMealByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}

pub fn get_meal_by_id_endpoint_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        API_V1_MENU_GET_BY_ID,
        web::get().to(get_meal_by_id_endpoint::<GetMealByIdUseCase>),
    );
}
