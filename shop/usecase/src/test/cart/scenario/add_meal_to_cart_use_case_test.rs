use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::{
    main::cart::value_objects::cart_id::{CartId, CartIdGenerator},
    test_fixtures::{rnd_cart_with_customer_id, rnd_customer_id, rnd_meal},
};

use crate::{
    main::cart::{
        add_meal_to_cart::{AddMealToCart, AddMealToCartUseCaseError},
        scenarios::add_meal_to_cart_use_case::AddMealToCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockCartPersister, MockMealExtractor},
};
