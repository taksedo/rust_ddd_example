use crate::main::endpoint_url::API_V1_MENU_GET_BY_ID;
use crate::main::menu::to_error::ToRestError;
use crate::main::menu::validation::Validated;
use actix_web::{http, web, HttpResponse};
use bigdecimal::BigDecimal;
use common_rest::main::rest_responses::{created, rest_business_error, BASE_URL};
use derive_new::new;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use http::Uri;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};

#[derive(new, Serialize, Deserialize, Debug)]
pub struct MealStruct {
    name: String,
    description: String,
    price: BigDecimal,
}

pub async fn execute<T>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    request: web::Json<MealStruct>,
) -> HttpResponse
where
    T: AddMealToMenu + Send + Debug,
{
    println!("Request {request:?} to add meal to menu received");

    let meal_name = MealName::validated(request.name.clone());
    let meal_description = MealDescription::validated(request.description.clone());
    let price = Price::validated(request.price.clone());

    let result = shared_state.lock().unwrap().execute(
        meal_name.unwrap(),
        meal_description.unwrap(),
        price.unwrap(),
    );

    match result {
        Ok(_) => created(
            API_V1_MENU_GET_BY_ID
                .clone()
                .replace("{id}", result.unwrap().value.to_string().as_str())
                .as_str()
                .parse::<Uri>()
                .unwrap(),
        ),
        Err(e) => e.to_rest_error(),
    }
}

impl ToRestError for AddMealToMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        rest_business_error("Meal already exists", "already_exists")
    }
}
