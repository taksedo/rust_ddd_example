use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_meal, rnd_meal_id};

use crate::{
    main::menu::{
        remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
        scenario::remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase,
    },
    test_fixtures::{MockMealExtractor, MockMealPersister},
};
