#![allow(non_snake_case)]

use crate::{
    main::menu::{meal::Meal, meal_restorer::MealRestorer},
    test_fixtures::{rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price, version},
};

#[test]
fn restore_meal__success() {
    let mealId = &rnd_meal_id();
    let name = &rnd_meal_name();
    let description = &rnd_meal_description();
    let price = &rnd_price();
    let removed = &true;
    let version = &version();

    let mut meal: Meal =
        MealRestorer::restore_meal(mealId, name, description, price, *removed, version, vec![]);

    assert_eq!(meal.id(), mealId);
    assert_eq!(meal.name(), name);
    assert_eq!(meal.removed(), removed);
    assert_eq!(meal.version(), version);
    assert_eq!(meal.pop_events().len(), 0)
}
