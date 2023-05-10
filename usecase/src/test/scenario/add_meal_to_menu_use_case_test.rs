use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuRequest};
use crate::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use crate::test_fixtures::fixtures::{TestEvent, TestMealPersister};
use common_types::main::base::domain_event::DomainEventTrait;
use core::any::Any;
use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_id;
use domain::test_fixtures::fixtures::rnd_meal_name;
use std::fmt::Debug;

#[test]
fn successfully_added() {
    type E = TestEvent;

    let rnd_meal_name = rnd_meal_name();

    let mut meal_persister = TestMealPersister::<E>::new();

    let id_generator = TestMealIdGenerator::new();

    let mut add_to_menu_use_case = AddMealToMenuUseCase::new(&mut meal_persister, &id_generator);
    let result = &add_to_menu_use_case
        .execute(AddMealToMenuRequest::new(rnd_meal_name.clone()))
        .unwrap();

    let meal_id = add_to_menu_use_case.id_generator.id;

    assert_eq!(result, &meal_id);

    let meal_from_usecase = meal_persister.value.get(&meal_id).unwrap();

    assert_eq!(&meal_from_usecase.id, &meal_id);
    assert_eq!(&meal_from_usecase.name, &rnd_meal_name);
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

    fn get_id(&self) -> &MealId {
        &self.id
    }
}
