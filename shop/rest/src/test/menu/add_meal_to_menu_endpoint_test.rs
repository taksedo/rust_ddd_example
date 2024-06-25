use std::sync::{Arc, Mutex};

use actix_web::{
    body::MessageBody,
    http::{header, StatusCode},
    web::{Data, Json},
};
use bigdecimal::{num_bigint::BigInt, BigDecimal, ToPrimitive};
use common::common_rest::rest_responses::{
    bad_request_type_url, error_type_url, GenericErrorResponse,
};
use domain::test_fixtures::{rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price};
use dotenvy::dotenv;
use usecase::main::menu::add_meal_to_menu::AddMealToMenuUseCaseError;

use crate::{
    main::{
        endpoint_url::API_V1_MENU_GET_BY_ID,
        menu::{add_meal_to_menu_endpoint, add_meal_to_menu_endpoint::AddMealToMenuRestRequest},
    },
    test::menu::add_meal_to_menu_endpoint_test::add_meal_to_menu_endpoint::add_meal_to_menu_endpoint,
    test_fixtures::{MockAddMealToMenu, StringMethodsForRestTestExt},
};
