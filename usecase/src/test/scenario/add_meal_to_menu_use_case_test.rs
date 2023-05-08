use crate::main::menu::add_meal_to_menu::{AddMealToMenu, AddMealToMenuRequest};
use crate::main::menu::scenario::add_meal_to_menu_use_case::AddMealToMenuUseCase;
use crate::test_fixtures::fixtures::TestMealPersister;
use derive_new::new;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_id;
use domain::test_fixtures::fixtures::rnd_meal_name;
use std::rc::Rc;

#[test]
fn successfully_added() {
    let rnd_meal_name = rnd_meal_name();
    let meal_persister = TestMealPersister::new();
    let id_generator = TestMealIdGenerator::new();
    let mut add_to_menu_use_case =
        AddMealToMenuUseCase::new(Box::new(meal_persister), Rc::new(id_generator));
    let result = add_to_menu_use_case
        .execute(AddMealToMenuRequest::new(rnd_meal_name))
        .unwrap()
    let id_generator = &add_to_menu_use_case.id_generator.to_owned();
    dbg!(id_generator);
    // dbg!(&meal_persister);
    // dbg!(&id);
    // assert_eq!(result, id.to_owned());

    // let meal = meal_persister.value.get(&id.to_owned());
    // println!("meal => {:?}", meal);
    // assert!(meal.is_some());
}

#[derive(new, Default, Debug, Clone)]
pub(crate) struct TestMealIdGenerator {
    #[new(value = "rnd_meal_id()")]
    id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&self) -> MealId {
        self.id
    }
}
