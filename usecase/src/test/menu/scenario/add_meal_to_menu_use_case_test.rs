use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use derive_new::new;
use domain::{
    main::menu::value_objects::meal_id::{MealId, MealIdGenerator},
    test_fixtures::{
        rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price, TestMealAlreadyExists,
    },
};

use crate::{
    main::menu::{
        add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError},
        scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase,
    },
    test_fixtures::MockMealPersister,
};

#[test]
fn successfully_added() {
    let name = rnd_meal_name();
    let description = rnd_meal_description();
    let price = rnd_price();
    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let meal_persister = Arc::new(Mutex::new(MockMealPersister::new()));

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        Arc::clone(&meal_persister) as _,
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(TestMealAlreadyExists { value: false })),
    );
    let result = add_to_menu_use_case.execute(name.clone(), description.clone(), price.clone());

    let id = id_generator.lock().unwrap().id;

    assert_eq!(result.unwrap(), id.to_owned());

    meal_persister.lock().unwrap().verify_invoked(
        Some(&id),
        Some(&name),
        Some(&description),
        Some(&price),
    );
}

#[test]
fn meal_already_exists() {
    let name = rnd_meal_name();
    let description = rnd_meal_description();
    let price = rnd_price();

    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let persister = Arc::new(Mutex::new(MockMealPersister::new()));

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        persister,
        id_generator,
        Arc::new(Mutex::new(TestMealAlreadyExists { value: true })),
    );
    let result = add_to_menu_use_case.execute(name, description, price);

    assert_eq!(result, Err(AddMealToMenuUseCaseError::AlreadyExists));
}

#[derive(new, Default, Debug, Clone, PartialEq)]
pub struct TestMealIdGenerator {
    #[new(value = "rnd_meal_id()")]
    id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&mut self) -> MealId {
        self.id
    }
}
