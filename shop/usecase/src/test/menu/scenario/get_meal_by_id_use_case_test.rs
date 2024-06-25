use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_meal, rnd_meal_id};

use crate::{
    main::menu::{
        dto::meal_info::MealInfo,
        get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
        scenario::get_meal_by_id_use_case::GetMealByIdUseCase,
    },
    test_fixtures::{removed_meal, MockMealExtractor},
};
