use crate::main::menu::validation::Validated;
use crate::main::start_web_backend;
use actix_web::{delete, get, post, web, Error, HttpResponse, HttpServer, Responder};
use application::main::event::event_publisher_impl::EventPublisherImpl;
use common_types::main::base::domain_entity::DomainEntity;
use derive_new::new;
use domain::main::menu::meal::{Meal, MealError};
use domain::main::menu::meal_events::DomainEventEnum;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::{CreateMealNameError, MealName};
use in_memory_persistence::main::menu::in_memory_incremental_meal_id_generator::InMemoryIncrementalMealIdGenerator;
use in_memory_persistence::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::ptr::addr_of_mut;
use std::rc::Rc;
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;
use usecase::main::menu::add_meal_to_menu::AddMealToMenu;
use usecase::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use usecase::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;

#[derive(Debug, Deserialize, Serialize)]
struct AddMealToMenuRequest {
    name: String,
}

struct AddMealToMenuEndpoint {
    add_meal_to_menu: dyn AddMealToMenu,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MealStruct {
    name: String,
}

// #[post("/add")]
pub async fn execute(
    // mut add_meal_to_menu: web::Data<AddMealToMenuUseCase>,
    request: web::Json<MealStruct>,
) -> String {
    println!("Request {request:?} to add meal to menu received");

    let meal_id_generator = Rc::new(RefCell::new(InMemoryIncrementalMealIdGenerator::new()));
    let meal_publisher = EventPublisherImpl::<DomainEventEnum>::default();
    let meal_repository = InMemoryMealRepository::new(Rc::new(RefCell::new((meal_publisher))));

    let meal_repository_ref = Rc::new(RefCell::new(meal_repository));

    let rule = MealAlreadyExistsUsesMealExtractor::new(Rc::clone(&meal_repository_ref) as _);

    dbg!(&meal_repository_ref);

    let mut add_meal_to_menu = AddMealToMenuUseCase::new(
        Rc::clone(&meal_repository_ref) as _,
        meal_id_generator,
        Rc::new(RefCell::new(rule)),
    );
    let meal_id = MealName::validated(request.name.clone())
        .map(|meal_name| add_meal_to_menu.execute(meal_name.clone()))
        .unwrap()
        .map(|adding_meal_to_menu_result| adding_meal_to_menu_result)
        .unwrap();
    dbg!(&meal_repository_ref);
    let meal = meal_repository_ref
        .borrow_mut()
        .get_by_id(meal_id.clone())
        .unwrap();
    dbg!(&meal);
    format!("=========={meal:?}=========")
    // web::Json(meal)
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
