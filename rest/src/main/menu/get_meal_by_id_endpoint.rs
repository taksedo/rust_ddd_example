use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};

use common_rest::main::rest_responses::resource_not_found;
use domain::main::menu::value_objects::meal_id::MealId;
use usecase::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};

use crate::main::menu::meal_model::MealModel;
use crate::main::menu::to_error::ToRestError;

pub async fn execute<T: GetMealById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let result = shared_state
        .lock()
        .unwrap()
        .execute(MealId::try_from(id).unwrap());

    match result {
        Ok(meal_info) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&MealModel::from(meal_info)).unwrap()),
        Err(e) => e.to_rest_error(),
    }
}

impl ToRestError for GetMealByIdUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}
