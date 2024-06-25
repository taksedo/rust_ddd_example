use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::rest_responses::{not_found_type_url, GenericErrorResponse};
use domain::{main::menu::value_objects::meal_id::MealId, test_fixtures::rnd_meal_id};
use dotenvy::dotenv;
use usecase::main::menu::get_meal_by_id::GetMealByIdUseCaseError::MealNotFound;

use crate::{
    main::menu::{get_meal_by_id_endpoint::get_meal_by_id_endpoint, meal_model::MealModel},
    test_fixtures::{rnd_meal_info, MockGetMealById},
};
