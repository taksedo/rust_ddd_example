use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::types::test_fixtures::rnd_count;
use domain::test_fixtures::{rnd_cart_with_customer_id_and_meals, rnd_customer_id, rnd_meal};

use crate::{
    main::cart::{
        get_cart::{CartItem, GetCart, GetCartUseCaseError},
        scenarios::get_cart_use_case::GetCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockMealExtractor},
};
