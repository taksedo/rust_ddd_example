use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use common_rest::main::rest_responses::resource_not_found;
use domain::main::menu::value_objects::meal_id::MealId;
use usecase::main::menu::remove_meal_from_menu::{
    RemoveMealFromMenu, RemoveMealFromMenuUseCaseError,
};

use crate::main::menu::to_error::ToRestError;

pub async fn execute<T: RemoveMealFromMenu + Send + Debug>(
    shared_state: web::Data<Arc<Mutex<T>>>,
    req: HttpRequest,
) -> HttpResponse {
    let id: i64 = req.match_info().get("id").unwrap().parse().unwrap();

    let meal_id = MealId::try_from(id);

    let result = shared_state.lock().unwrap().execute(meal_id.unwrap());
    match result {
        Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
        Err(e) => e.to_rest_error(),
    }
}

impl ToRestError for RemoveMealFromMenuUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        resource_not_found()
    }
}
