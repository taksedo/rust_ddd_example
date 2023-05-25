use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use crate::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use crate::test_fixtures::fixtures::MockMealPersister;
use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_name;
use domain::test_fixtures::fixtures::{rnd_meal_id, TestMealAlreadyExists};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[test]
fn successfully_added() {
    let name = rnd_meal_name();
    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let persister = Arc::new(Mutex::new(MockMealPersister::new()));
    let persister_binding = Arc::clone(&persister);

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        persister,
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(TestMealAlreadyExists { value: false })),
    );
    let result = add_to_menu_use_case.execute(name.clone());

    let id = id_generator.lock().unwrap().id;

    assert_eq!(result.unwrap(), id.to_owned());

    let persister_clone = persister_binding.lock().unwrap();
    Arc::new(persister_clone).verify_invoked(Some(id), Some(name));
}

#[test]
fn meal_already_exists() {
    let name = rnd_meal_name();

    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let persister = Arc::new(Mutex::new(MockMealPersister::new()));

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        persister,
        id_generator,
        Arc::new(Mutex::new(TestMealAlreadyExists { value: true })),
    );
    let result = add_to_menu_use_case.execute(name);

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
