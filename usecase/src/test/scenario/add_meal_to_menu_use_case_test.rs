use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuUseCaseError};
use crate::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use crate::test_fixtures::fixtures::MockMealPersister;
use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_name;
use domain::test_fixtures::fixtures::{rnd_meal_id, TestMealAlreadyExists};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[test]
fn successfully_added() {
    let name = rnd_meal_name();
    let id_generator = Rc::new(TestMealIdGenerator::new());
    let persister = Rc::new(RefCell::new(MockMealPersister::new()));
    let persister_binding = Rc::clone(&persister);

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        persister,
        id_generator.clone(),
        Rc::new(RefCell::new(TestMealAlreadyExists { value: false })),
    );
    let result = add_to_menu_use_case.execute(name.clone());

    let id = id_generator.id;

    assert_eq!(result.unwrap(), id.to_owned());

    let persister_clone = persister_binding.borrow();
    Rc::new(persister_clone).verify_invoked(Some(id), Some(name));
}

#[test]
fn meal_already_exists() {
    let name = rnd_meal_name();

    let id_generator = Rc::new(TestMealIdGenerator::new());
    let persister = Rc::new(RefCell::new(MockMealPersister::new()));

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
        persister,
        id_generator,
        Rc::new(RefCell::new(TestMealAlreadyExists { value: true })),
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
    fn generate(&self) -> MealId {
        self.id
    }
}
