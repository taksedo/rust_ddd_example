use std::cell::RefCell;
use std::rc::Rc;
use usecase::main::menu::add_meal_to_menu::{
    AddMealToMenu, AddMealToMenuRequest, InvalidMealParametersError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddMealToMenuRestRequest {
    name: String,
}

// impl MenuController {
//     fn menu(model_map: ) {
//
//     }
// }

use actix_web::{delete, get, post, web, Error, HttpResponse, HttpServer, Responder};
use derive_new::new;
use domain::main::menu::meal_name::MealName;
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use serde::{Deserialize, Serialize};
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("Hello world is happening");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MealStruct {
    name: String,
}

#[post("/add")]
pub async fn add_meal_to_menu(request: web::Json<MealStruct>) -> String {
    println!("Request {request:?} to add meal to menu received");
    MealName::from(request.name.clone())
        .map_err(|_| InvalidMealParametersError::InvalidParameters)
        .map(AddMealToMenuRequest::new)
        .unwrap();

    let add_meal_to_meanu_request = AddMealToMenuRequest::from(request.name.clone()).unwrap();

    // dbg!(add_meal_to_meanu_request);
    format!("Meal '' successfully added" /*meal_name*/,)
}

// #[delete("/remove/{id}")]
// pub async fn remove_meal_from_menu(mealId: u64) {}

/// deserialize `Info` from request's body
#[post("/submit/info")]
pub async fn info(info: web::Json<Info>) -> web::Json<Info> {
    println!("=========={info:?}=========");
    web::Json(Info {
        username: info.username.clone(),
        email: info.email.clone(),
        password: info.password.clone(),
        confirm_password: info.confirm_password.clone(),
    })
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}
