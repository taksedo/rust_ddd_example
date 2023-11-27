use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_meal, rnd_meal_id};

use crate::main::menu::dto::meal_info::MealInfo;
use crate::main::menu::get_meal_by_id::{GetMealById, GetMealByIdUseCaseError};
use crate::main::menu::scenario::get_meal_by_id_use_case::GetMealByIdUseCase;
use crate::test_fixtures::{removed_meal, MockMealExtractor};

#[test]
fn meal_not_found() {
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let mut use_case = GetMealByIdUseCase::new(meal_extractor);

    let meal_id = rnd_meal_id();
    let result = use_case.execute(meal_id);

    assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_id(&meal_id);
}

#[test]
fn meal_removed() {
    let meal = removed_meal();
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor {
        meal: Option::from(meal.to_owned()),
        ..MockMealExtractor::default()
    }));

    let mut use_case = GetMealByIdUseCase::new(meal_extractor);
    let result = use_case.execute(meal.entity_params.id);

    assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_id(&meal.entity_params.id);
}

#[test]
fn meal_extracted_successfully() {
    let meal = rnd_meal();
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor {
        meal: Option::from(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut use_case = GetMealByIdUseCase::new(meal_extractor);

    let result = use_case.execute(meal.entity_params.id);
    let meal_info = result;

    assert_eq!(
        meal_info.unwrap(),
        MealInfo {
            id: meal.entity_params.id,
            name: meal.name,
            description: meal.description,
            price: meal.price,
            version: meal.entity_params.version,
        }
    );
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_id(&meal.entity_params.id);
}
