use std::sync::{Arc, Mutex};

use domain::test_fixtures::rnd_meal;

use crate::{
    main::menu::{
        dto::meal_info::MealInfo, get_menu::GetMenu, scenario::get_menu_use_case::GetMenuUseCase,
    },
    test_fixtures::MockMealExtractor,
};
