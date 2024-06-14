use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::types::test_fixtures::rnd_count;
use domain::test_fixtures::{rnd_cart, rnd_customer_id, rnd_meal};

use crate::{
    main::cart::{
        get_cart::{CartItem, GetCart, GetCartUseCaseError},
        scenarios::get_cart_use_case::GetCartUseCase,
    },
    test_fixtures::{MockCartExtractor, MockMealExtractor},
};

#[test]
fn cart_successfully_extracted() {
    let meal = rnd_meal();

    let count = rnd_count();

    let customer_id = rnd_customer_id();

    let mut cart = rnd_cart();
    cart.for_customer = customer_id;
    cart.meals = HashMap::from([(*meal.get_id(), count)]);

    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    cart_extractor.lock().unwrap().cart = Some(cart.clone());

    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    meal_extractor.lock().unwrap().meal = Some(meal.clone());

    let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
    let result = use_case.execute(customer_id);

    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(&cart.for_customer);
    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());
    let extracted_cart = result.unwrap();
    assert_eq!(extracted_cart.for_customer, customer_id);
    assert_eq!(
        extracted_cart.items,
        vec![CartItem::new(
            *meal.get_id(),
            meal.get_name().to_owned(),
            count
        )]
    )
}

#[test]
fn cart_not_found() {
    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
    let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
    let customer_id = rnd_customer_id();

    let result = use_case.execute(customer_id);

    cart_extractor.lock().unwrap().verify_invoked(&customer_id);
    meal_extractor.lock().unwrap().verify_empty();
    assert_eq!(result.unwrap_err(), GetCartUseCaseError::CartNotFound);
}
