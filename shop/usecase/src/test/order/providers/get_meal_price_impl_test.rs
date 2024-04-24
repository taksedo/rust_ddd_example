use std::sync::{Arc, Mutex};

use assert_panic::assert_panic;
use domain::{
    main::order::get_meal_price::GetMealPrice,
    test_fixtures::{rnd_meal, rnd_meal_id},
};

use crate::{
    main::order::providers::get_meal_price_using_extractor::GetMealPriceUsingExtractor,
    test_fixtures::MockMealExtractor,
};

#[test]
fn price_has_been_provided() {
    let meal = rnd_meal();

    let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    extractor.lock().unwrap().meal = Some(meal.clone());

    let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());
    let result = get_meal_price.invoke(meal.entity_params.id);

    extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.entity_params.id);
    assert_eq!(result, meal.price);
}

#[test]
fn meal_not_found() {
    let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let get_meal_price = GetMealPriceUsingExtractor::new(extractor.clone());

    let meal_id = rnd_meal_id();

    assert_panic!( {get_meal_price.invoke(meal_id);}, String, starts with &format!("Meal #{:?} not found", meal_id));

    extractor.lock().unwrap().verify_invoked_get_by_id(&meal_id);
}
