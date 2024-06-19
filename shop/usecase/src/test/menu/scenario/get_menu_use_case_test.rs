use std::sync::{Arc, Mutex};

use domain::test_fixtures::rnd_meal;

use crate::{
    main::menu::{
        dto::meal_info::MealInfo, get_menu::GetMenu, scenario::get_menu_use_case::GetMenuUseCase,
    },
    test_fixtures::MockMealExtractor,
};

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
            id: *meal.id(),
            name: meal.name().to_owned(),
            description: meal.description().to_owned(),
            price: meal.price().to_owned(),
            version: *meal.version(),
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
