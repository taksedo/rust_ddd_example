use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_meal, rnd_meal_id};

use crate::{
    main::menu::{
        remove_meal_from_menu::{RemoveMealFromMenu, RemoveMealFromMenuUseCaseError},
        scenario::remove_meal_from_menu_use_case::RemoveMealFromMenuUseCase,
    },
    test_fixtures::{MockMealExtractor, MockMealPersister},
};

#[test]
fn successfully_removed() {
    let meal = rnd_meal();

    let meal_persister = Arc::new(Mutex::new(MockMealPersister::new()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    meal_extractor.lock().unwrap().meal = Some(meal.clone());

    let mut use_case =
        RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());
    let result = use_case.execute(meal.get_id());

    assert!(result.is_ok());

    let meal = meal_persister.lock().unwrap().meal.clone().unwrap();
    //todo: придумать более изящное тестирование meal

    meal_persister.lock().unwrap().verify_invoked_meal(&meal);

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());

    meal_persister
        .lock()
        .unwrap()
        .verify_events_after_deletion(&meal.get_id());
}

#[test]
fn meal_not_found() {
    let meal_persister = Arc::new(Mutex::new(MockMealPersister::new()));
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let mut use_case =
        RemoveMealFromMenuUseCase::new(meal_extractor.clone(), meal_persister.clone());

    let meal_id = rnd_meal_id();

    let result = use_case.execute(&meal_id);

    assert_eq!(result, Err(RemoveMealFromMenuUseCaseError::MealNotFound));
    meal_persister.lock().unwrap().verify_empty();

    meal_extractor
        .lock()
        .unwrap()
        .verify_invoked_get_by_id(&meal_id);
}
