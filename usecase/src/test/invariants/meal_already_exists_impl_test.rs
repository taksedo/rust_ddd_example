use crate::main::menu::invariant::meal_already_exists_impl::MealAlreadyExistsImpl;
use crate::test_fixtures::fixtures::{removed_meal, TestMealExtractor};
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_already_exists::MealAlreadyExists;
use domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent;
use domain::test_fixtures::fixtures::{rnd_meal, rnd_meal_name};

#[test]
fn meal_already_exist() {
    type E = MealRemovedFromMenuDomainEvent;
    let meal: Meal<E> = rnd_meal();
    let mut extractor = TestMealExtractor::<E>::new();
    extractor.value.insert(meal.id, meal.clone());
    let mut rule = MealAlreadyExistsImpl::new(extractor);
    let result = rule.check(meal.name);

    assert!(result);
}

#[test]
fn meal_already_exists_but_removed() {
    type E = MealRemovedFromMenuDomainEvent;
    let meal: Meal<E> = removed_meal();
    let mut extractor: TestMealExtractor<E> = TestMealExtractor::new();
    extractor.value.insert(meal.id, meal.clone());
    let mut rule = MealAlreadyExistsImpl::new(extractor);

    let result = rule.check(meal.name);

    assert!(!result);
}

#[test]
fn meal_already_exists_doesnt_exist() {
    type E = MealRemovedFromMenuDomainEvent;
    let extractor: TestMealExtractor<E> = TestMealExtractor::new();
    let mut rule = MealAlreadyExistsImpl::new(extractor);
    let result = rule.check(rnd_meal_name());
    assert!(!result)
}
