use std::{
    fmt::Debug,
    str::FromStr,
    sync::{Arc, Mutex},
};

use actix_web::{http, web, HttpResponse};
use bigdecimal::BigDecimal;
use common::common_rest::main::rest_responses::{
    created, rest_business_error, to_invalid_param_bad_request,
};
use derive_new::new;
use domain::main::menu::value_objects::{
    meal_description::MealDescription, meal_name::MealName, price::Price,
};
use http::Uri;
use serde::{Deserialize, Serialize};
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};

use crate::main::{
    endpoint_url::API_V1_MENU_GET_BY_ID, to_error::ToRestError, validated::Validated,
};

#[derive(new, Serialize, Deserialize, Debug)]
pub struct AddMealToMenuRestRequest {
    name: String,
    description: String,
    price: f64,
}

pub async fn execute<T>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    request: web::Json<AddMealToMenuRestRequest>,
) -> HttpResponse
where
    T: AddMealToMenu + Send + Debug,
{
    println!("Request {request:?} to add meal to menu received");

    let error_list = Arc::new(Mutex::new(vec![]));

    match (
        MealName::validated(&request.name, error_list.clone()),
        MealDescription::validated(&request.description, error_list.clone()),
        Price::validated(
            BigDecimal::from_str(&request.price.to_string()).unwrap(),
            error_list.clone(),
        ),
    ) {
        (Ok(meal_name), Ok(meal_description), Ok(price)) => {
            match shared_state
                .lock()
                .unwrap()
                .execute(meal_name, meal_description, price)
            {
                Ok(meal_id) => created(
                    API_V1_MENU_GET_BY_ID
                        .replace("{id}", &meal_id.to_i64().to_string())
                        .as_str()
                        .parse::<Uri>()
                        .unwrap(),
                ),
                Err(e) => e.to_rest_error(),
            }
        }
        (_, _, _) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for AddMealToMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        rest_business_error("Meal already exists", "already_exists")
    }
}
