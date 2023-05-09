#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuRequest};
use crate::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use crate::test_fixtures::fixtures::TestMealPersister;
use core::any::Any;
use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_id;
use domain::test_fixtures::fixtures::rnd_meal_name;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[test]
fn successfully_added() {
    let rnd_meal_name = rnd_meal_name();

    let meal_persister = TestMealPersister::new();

    let id_generator = TestMealIdGenerator::new();

    // let id_watcher = &id_generator.id;

    let mut add_to_menu_use_case =
        AddMealToMenuUseCase::new(Box::new(meal_persister), Rc::new(id_generator));
    let result = &add_to_menu_use_case
        .execute(AddMealToMenuRequest::new(rnd_meal_name))
        .unwrap();

    let id_generator = add_to_menu_use_case.id_generator.clone();

    let id = TestMealIdGenerator { id: *result };

    assert_eq!(*id_generator, id);
}

#[derive(new, Default, Debug, Clone, PartialEq)]
pub(crate) struct TestMealIdGenerator {
    #[new(value = "rnd_meal_id()")]
    id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&self) -> MealId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
