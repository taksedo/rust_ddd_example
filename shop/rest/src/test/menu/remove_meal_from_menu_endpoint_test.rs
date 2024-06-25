use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
use common::common_rest::rest_responses::{not_found_type_url, GenericErrorResponse};
use domain::test_fixtures::rnd_meal_id;
use dotenvy::dotenv;
use usecase::main::menu::remove_meal_from_menu::RemoveMealFromMenuUseCaseError;

use crate::{
    main::menu::remove_meal_from_menu_endpoint::remove_meal_from_menu_endpoint,
    test_fixtures::MockRemoveMealFromMenu,
};
