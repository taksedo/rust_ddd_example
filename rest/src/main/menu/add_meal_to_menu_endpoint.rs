use crate::main::endpoint_url::MENU_GET_BY_ID;
use crate::main::menu::validation::Validated;
use actix_web::{get, post, web, HttpResponse, Responder};
use derive_new::new;
use domain::main::menu::meal_name::MealName;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};

#[derive(Debug, new)]
pub struct AddMealToMenuEndPointSharedState<T: AddMealToMenu + Send + Debug> {
    pub add_meal_to_menu: Arc<Mutex<T>>,
}

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

#[derive(new, Serialize, Deserialize, Debug)]
pub struct MealStruct {
    name: String,
}

pub async fn execute<T>(
    shared_state: web::Data<AddMealToMenuEndPointSharedState<T>>,
    request: web::Json<MealStruct>,
) -> String
where
    T: AddMealToMenu + Send + Debug,
{
    let add_meal_to_menu = &shared_state.add_meal_to_menu;
    println!("Request {request:?} to add meal to menu received");

    let meal_id = MealName::validated(request.name.clone())
        .map(|meal_name| add_meal_to_menu.lock().unwrap().execute(meal_name.clone()))
        .unwrap_or_else(|_e| Err(AddMealToMenuUseCaseError::AlreadyExists))
        .map(|adding_meal_to_menu_result| adding_meal_to_menu_result)
        .unwrap();
    println!("{}", MENU_GET_BY_ID);
    // dbg!(&add_meal_to_menu);
    format!("=========={meal_id:?}=========")
}

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
