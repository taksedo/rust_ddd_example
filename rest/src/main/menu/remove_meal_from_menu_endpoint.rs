use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use domain::main::menu::value_objects::meal_id::MealId;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use usecase::main::menu::remove_meal_from_menu::RemoveMealFromMenu;

pub async fn execute<T: RemoveMealFromMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let meal_id = MealId::from(id);

    shared_state.lock().unwrap().execute(meal_id.unwrap())?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(""))
}
