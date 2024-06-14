use std::sync::{Arc, Mutex};

use domain::test_fixtures::{rnd_meal, rnd_meal_id};

use crate::{
    main::menu::{
        dto::meal_info::MealInfo,
        get_meal_by_id::{GetMealById, GetMealByIdUseCaseError},
        scenario::get_meal_by_id_use_case::GetMealByIdUseCase,
    },
    test_fixtures::{removed_meal, MockMealExtractor},
};

#[test]
fn meal_not_found() {
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
    let mut use_case = GetMealByIdUseCase::new(meal_extractor);

    let meal_id = &rnd_meal_id();
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
    let result = use_case.execute(meal.get_id());

    assert_eq!(result, Err(GetMealByIdUseCaseError::MealNotFound));
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_id(meal.get_id());
}

#[test]
fn meal_extracted_successfully() {
    let meal = rnd_meal();
    let meal_extractor = Arc::new(Mutex::new(MockMealExtractor {
        meal: Option::from(meal.to_owned()),
        ..MockMealExtractor::default()
    }));
    let mut use_case = GetMealByIdUseCase::new(meal_extractor);

    let result = use_case.execute(meal.get_id());
    let meal_info = result;

    assert_eq!(
        meal_info.unwrap(),
        MealInfo {
            id: *meal.get_id(),
            name: meal.get_name().to_owned(),
            description: meal.get_description().to_owned(),
            price: meal.get_price().to_owned(),
            version: *meal.get_version(),
        }
    );
    use_case
        .meal_extractor
        .lock()
        .unwrap()
        .downcast_ref::<MockMealExtractor>()
        .unwrap()
        .verify_invoked_get_by_id(&meal.get_id());
}
