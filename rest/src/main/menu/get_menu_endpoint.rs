use crate::main::menu::meal_model::MealModel;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Result};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::get_menu::GetMenu;

pub async fn execute<T: GetMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
) -> Result<HttpResponse> {
    #[allow(clippy::redundant_closure)]
    let meal_info_list: Vec<MealModel> = shared_state
        .lock()
        .unwrap()
        .execute()
        .into_iter()
        .map(|meal_info| MealModel::from(meal_info))
        .collect();

    let resp = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&meal_info_list)?);

    Ok(resp)
}
