use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use actix_web::{http::header::ContentType, web, HttpResponse};
use usecase::main::menu::{get_menu::GetMenu, scenario::get_menu_use_case::GetMenuUseCase};

use crate::main::{endpoint_url::API_V1_MENU_GET_ALL, menu::meal_model::MealModel};

pub async fn get_menu_endpoint<T: GetMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
) -> HttpResponse {
    let meal_model_list: Vec<MealModel> = shared_state
        .lock()
        .unwrap()
        .execute()
        .into_iter()
        .map(MealModel::from)
        .collect();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&meal_model_list).unwrap())
}

pub fn get_menu_endpoint_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        API_V1_MENU_GET_ALL,
        web::get().to(get_menu_endpoint::<GetMenuUseCase>),
    );
}
