use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use derive_new::new;
use domain::{
    main::menu::value_objects::meal_id::{MealId, MealIdGenerator},
    test_fixtures::{
        rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price, TestMealAlreadyExists,
    },
};

use crate::{
    main::menu::{
        add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError},
        scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase,
    },
    test_fixtures::MockMealPersister,
};
