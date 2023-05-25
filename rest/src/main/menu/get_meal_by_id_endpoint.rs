use crate::main::menu::meal_model::MealModel;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use derive_new::new;
use domain::main::menu::meal_id::MealId;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::get_meal_by_id::GetMealById;

#[derive(Debug, new)]
pub struct GetMealByIdEndpointSharedState<T: GetMealById + Send + Debug> {
    pub meal_get_by_id: Arc<Mutex<T>>,
}

pub async fn execute<T: GetMealById + Send + Debug>(
    shared_state: web::Data<GetMealByIdEndpointSharedState<T>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id: u64 = req.match_info().get("id").unwrap().parse().unwrap();

    let get_meal_by_id_use_case = &shared_state.meal_get_by_id;
    let meal_info = get_meal_by_id_use_case
        .lock()
        .unwrap()
        .execute(MealId::new(id))
        .map_err(|e| e)?;
    let resp = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("{:?}", MealModel::from(meal_info)));

    Ok(resp)
}
