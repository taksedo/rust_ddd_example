#![allow(non_snake_case)]

use crate::main::menu::meal::Meal;
use crate::main::menu::meal_restorer::MealRestorer;
use crate::test_fixtures::fixtures::{
    rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price, version,
};
use common_types::main::base::domain_entity::DomainEntityTrait;

#[test]
fn restore_meal__success() {
    let mealId = rnd_meal_id();
    let name = rnd_meal_name();
    let description = rnd_meal_description();
    let price = rnd_price();
    let removed = true;
    let version = version();

    let meal: Meal = MealRestorer::restore_meal(
        mealId,
        name.clone(),
        description,
        price,
        removed,
        version,
        vec![],
    );

    assert_eq!(meal.domain_entity_field.id, mealId);
    assert_eq!(meal.name, name);
    assert_eq!(meal.removed, removed);
    assert_eq!(meal.domain_entity_field.version, version);
    dbg!(meal.pop_events());
    assert_eq!(meal.pop_events().len(), 0)
}
