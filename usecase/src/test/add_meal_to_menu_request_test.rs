use crate::main::menu::add_meal_to_menu::AddMealToMenuRequest;
use domain::test_fixtures::fixtures::rnd_meal_name;

#[test]
fn successfully_created() {
    let name = rnd_meal_name();
    let result = AddMealToMenuRequest::from(name.to_owned().value).unwrap();
    assert_eq!(result, AddMealToMenuRequest::new(name))
}

// #[test]
// fn successfully_added() {
//     let name = rnd_meal_name();
//     let id_generator = TestMealIdGenerator::new();
//
//     let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
//         persister,
//         Rc::new(id_generator.to_owned()),
//         TestMealAlreadyExists { value: false },
//     );
//     let result =
//         add_to_menu_use_case.execute(name.to_owned(), description.to_owned(), price.to_owned());
//
//     let id = id_generator.id;
//
//     assert_eq!(result.unwrap(), id.to_owned());
//
//     add_to_menu_use_case.meal_persister.verify_invoked(
//         Some(id),
//         Some(name),
//         Some(description),
//         Some(price),
//     );
// }
//
// #[test]
// fn meal_already_exists() {
//     let name = rnd_meal_name();
//     let description = rnd_meal_description();
//     let price = rnd_price();
//     let id_generator = TestMealIdGenerator::new();
//     let persister = MockMealPersister::new();
//
//     let mut add_to_menu_use_case = AddMealToMenuUseCase::new(
//         persister,
//         Rc::new(id_generator.to_owned()),
//         TestMealAlreadyExists { value: true },
//     );
//     let result =
//         add_to_menu_use_case.execute(name.to_owned(), description.to_owned(), price.to_owned());
//
//     assert_eq!(result, Err(AddMealToMenuUseCaseError::AlreadyExists));
// }
//
// #[derive(new, Default, Debug, Clone)]
// pub(crate) struct TestMealIdGenerator {
//     #[new(value = "rnd_meal_id()")]
//     id: MealId,
// }
//
// impl MealIdGenerator for TestMealIdGenerator {
//     fn generate(&self) -> MealId {
//         self.id
//     }
// }
