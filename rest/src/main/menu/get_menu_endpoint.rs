use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpResponse};
use usecase::main::menu::get_menu::GetMenu;

use crate::main::menu::meal_model::MealModel;

pub async fn execute<T: GetMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
) -> HttpResponse {
    let meal_info_list: Vec<MealModel> = shared_state
        .lock()
        .unwrap()
        .execute()
        .into_iter()
        .map(MealModel::from)
        .collect();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&meal_info_list).unwrap())
}
