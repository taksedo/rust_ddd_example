use crate::main::menu::validation::Validated;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Result};
use bigdecimal::BigDecimal;
use derive_new::new;
use domain::main::menu::value_objects::meal_description::MealDescription;
use domain::main::menu::value_objects::meal_name::MealName;
use domain::main::menu::value_objects::price::Price;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::add_meal_to_menu::AddMealToMenu;

#[derive(new, Serialize, Deserialize, Debug)]
pub struct MealStruct {
    name: String,
    description: String,
    price: BigDecimal,
}

pub async fn execute<T>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    request: web::Json<MealStruct>,
) -> Result<HttpResponse>
where
    T: AddMealToMenu + Send + Debug,
{
    println!("Request {request:?} to add meal to menu received");

    let meal_name = MealName::validated(request.name.clone())?;
    let meal_description = MealDescription::validated(request.description.clone())?;
    let price = Price::validated(request.price.clone())?;

    let meal_id = shared_state
        .lock()
        .unwrap()
        .execute(meal_name, meal_description, price)?;

    println!("{meal_id:?}");

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("Location", meal_id.to_i64()))
        .body(""))
}
