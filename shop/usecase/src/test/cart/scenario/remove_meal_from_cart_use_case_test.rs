use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_cart, rnd_meal_id};

use crate::{
    main::cart::{
        remove_meal_from_cart::{RemoveMealFromCart, RemoveMealFromCartUseCaseError},
        scenarios::remove_meal_from_cart_use_case::RemoveMealFromCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockCartPersister},
};
