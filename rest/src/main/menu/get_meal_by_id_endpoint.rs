use crate::main::menu::meal_model::MealModel;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use domain::main::menu::value_objects::meal_id::MealId;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::get_meal_by_id::GetMealById;

pub async fn execute<T: GetMealById + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let meal_info = shared_state
        .lock()
        .unwrap()
        .execute(MealId::try_from(id)?)?;
    let resp = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&MealModel::from(meal_info))?);

    Ok(resp)
}
