use std::sync::{Arc, Mutex};

use domain::{
    main::menu::meal_already_exists::MealAlreadyExists,
    test_fixtures::{rnd_meal, rnd_meal_name},
};

use crate::{
    main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor,
    test_fixtures::{removed_meal, MockMealExtractor},
};
