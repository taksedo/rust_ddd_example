use std::sync::{Arc, Mutex};

use actix_web::{body::MessageBody, web};

use crate::{
    main::menu::{get_menu_endpoint::get_menu_endpoint, meal_model::MealModel},
    test_fixtures::{rnd_meal_info, MockGetMenu},
};
