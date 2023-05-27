use crate::main::menu::meal_model::MealModel;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Result};
use derive_new::new;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::get_menu::GetMenu;

#[derive(Debug, new)]
pub struct GetMenuEndpointSharedState<T: GetMenu + Send + Debug> {
    pub meal_get_menu: Arc<Mutex<T>>,
}

pub async fn execute<T: GetMenu + Send + Debug>(
    shared_state: web::Data<GetMenuEndpointSharedState<T>>,
) -> Result<HttpResponse> {
    let get_menu_use_case = &shared_state.meal_get_menu;
    let meal_info_list: Vec<MealModel> = get_menu_use_case
        .lock()
        .unwrap()
        .execute()
        .into_iter()
        .map(|meal_info| MealModel::from(meal_info))
        .collect();
    let resp = HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("{:?}", meal_info_list));

    Ok(resp)
}
