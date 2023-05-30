use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_menu::GetMenu;
use crate::main::menu::scenario::get_menu_use_case::GetMenuUseCase;
use crate::test_fixtures::fixtures::MockMealExtractor;
use domain::test_fixtures::fixtures::rnd_meal;
use std::sync::{Arc, Mutex};

#[test]
#[allow(non_snake_case)]
fn get_menu__menu_is_empty() {
    let meal_extractor = MockMealExtractor::new();
    let use_case = GetMenuUseCase::new(Arc::new(Mutex::new(meal_extractor)));
    let menu = use_case.execute();

    assert!(menu.is_empty());
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_all();
}

#[test]
fn get_menu() {
    let meal = rnd_meal();
    let meal_extractor = MockMealExtractor {
        meal: Option::from(meal.to_owned()),
        ..MockMealExtractor::default()
    };
    let use_case = GetMenuUseCase::new(Arc::new(Mutex::new(meal_extractor)));
    let menu = use_case.execute();

    assert_eq!(
        menu,
        vec![MealInfo {
            id: meal.domain_entity_field.id,
            name: meal.name,
            description: meal.description,
            price: meal.price,
            version: meal.domain_entity_field.version,
        }]
    );
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_all();
}
