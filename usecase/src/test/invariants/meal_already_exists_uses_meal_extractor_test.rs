use crate::main::menu::invariant::meal_already_exists_uses_meal_extractor::MealAlreadyExistsUsesMealExtractor;
use crate::test_fixtures::fixtures::{removed_meal, MockMealExtractor};
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::test_fixtures::fixtures::{rnd_meal, rnd_meal_id, rnd_meal_name};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn meal_already_exists() {
    let meal = rnd_meal();
    let extractor = Rc::new(RefCell::new(MockMealExtractor {
        meal: Some(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let result = rule.invoke(&meal.name);

    assert!(result);

    rule.extractor
        .borrow_mut()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(meal.to_owned().name);
}
#[test]
fn meal_already_exists_but_removed() {
    let meal = removed_meal();
    let extractor = Rc::new(RefCell::new(MockMealExtractor {
        meal: Some(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let result = rule.invoke(&meal.name);

    assert!(!result);
    rule.extractor
        .borrow_mut()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(meal.to_owned().name);
}

#[test]
fn meal_already_exists_doesnt_exist() {
    let extractor = Rc::new(RefCell::new(MockMealExtractor::new()));
    let mut rule = MealAlreadyExistsUsesMealExtractor::new(extractor);

    let meal_name = rnd_meal_name();
    let result = rule.invoke(&meal_name);

    assert!(!result);
    rule.extractor
        .borrow_mut()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_name(meal_name);
}
