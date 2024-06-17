use std::sync::{Arc, Mutex};

use domain::{
    main::menu::meal_already_exists::MealAlreadyExists,
    test_fixtures::{rnd_meal, rnd_meal_name},
};

use crate::{
    main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor,
    test_fixtures::{removed_meal, MockMealExtractor},
};

#[test]
fn meal_already_exists() {
    let meal = rnd_meal();
    let extractor = Arc::new(Mutex::new(MockMealExtractor {
        meal: Some(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let result = rule.invoke(&meal.get_name());

    assert!(result);

    rule.extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(&meal.get_name());
}

#[test]
fn meal_already_exists_but_removed() {
    let meal = removed_meal();
    let extractor = Arc::new(Mutex::new(MockMealExtractor {
        meal: Some(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let result = rule.invoke(&meal.get_name());

    assert!(!result);
    rule.extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(&meal.get_name());
}

#[test]
fn meal_already_exists_doesnt_exist() {
    let extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let meal_name = rnd_meal_name();
    let result = rule.invoke(&meal_name);

    assert!(!result);
    rule.extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(&meal_name);
}
