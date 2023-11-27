#![allow(non_snake_case)]

use std::sync::atomic::AtomicI64;
use std::sync::{Arc, Mutex};

use common::types::main::base::domain_entity::DomainEntityTrait;
use derive_new::new;

use crate::main::menu::meal::Meal;
use crate::main::menu::meal::MealError::AlreadyExistsWithSameNameError;
use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_events::{
    DomainEventEnum, MealAddedToMenuDomainEvent, MealRemovedFromMenuDomainEvent,
};
use crate::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::value_objects::meal_name::MealName;
use crate::test_fixtures::{
    print_type_of, rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
};

#[derive(Debug, new, Default)]
pub(crate) struct TestMealIdGenerator {
    #[new(value = "AtomicI64::from(0)")]
    _counter: AtomicI64,
    #[new(value = "rnd_meal_id()")]
    pub meal_id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&mut self) -> MealId {
        self.meal_id
    }
}

#[derive(Debug, new, Default, Clone, Copy)]
pub struct TestMealAlreadyExists {
    #[new(value = "false")]
    pub value: bool,
}

impl MealAlreadyExists for TestMealAlreadyExists {
    fn invoke(&mut self, _name: &MealName) -> bool {
        self.value
    }
}

#[test]
fn add_meal__success() {
    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let meal_exists = Arc::new(Mutex::new(TestMealAlreadyExists { value: false }));
    let name = rnd_meal_name();
    let description = rnd_meal_description();
    let price = rnd_price();
    let result = Meal::add_meal_to_menu(
        Arc::clone(&id_generator) as _,
        meal_exists,
        name.to_owned(),
        description.to_owned(),
        price.to_owned(),
    );

    let mut test_meal = result.unwrap();
    assert_eq!(
        &test_meal.entity_params.id,
        &id_generator.lock().unwrap().meal_id
    );
    assert_eq!(test_meal.name, name);
    assert_eq!(test_meal.description, description);
    assert_eq!(test_meal.price, price);
    assert!(test_meal.visible());

    let popped_events = test_meal.entity_params.pop_events();
    let popped_events = popped_events.get(0).unwrap();

    let expected_event = &DomainEventEnum::MealAddedToMenuDomainEvent(
        MealAddedToMenuDomainEvent::new(id_generator.lock().unwrap().meal_id),
    );
    assert_eq!(
        print_type_of(&popped_events),
        print_type_of(&expected_event)
    );
}

#[test]
fn add_meal_to_menu__already_exists_with_the_same_name() {
    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
    let meal_exists = Arc::new(Mutex::new(TestMealAlreadyExists { value: true }));
    let name = rnd_meal_name();
    let description = rnd_meal_description();
    let price = rnd_price();
    let result = Meal::add_meal_to_menu(id_generator, meal_exists, name, description, price);

    assert_eq!(result.unwrap_err(), AlreadyExistsWithSameNameError);
}

#[test]
fn remove_meal_from_menu__success() {
    let mut test_meal = rnd_meal();
    test_meal.remove_meal_from_menu();
    assert!(test_meal.removed);
    assert!(!test_meal.visible());

    let popped_events = test_meal.entity_params.pop_events();
    let popped_events = popped_events.get(0).unwrap();

    let expected_event = &DomainEventEnum::MealRemovedFromMenuDomainEvent(
        MealRemovedFromMenuDomainEvent::new(test_meal.entity_params.id),
    );
    assert_eq!(
        print_type_of(&popped_events),
        print_type_of(&expected_event)
    );
}

#[test]
fn remove_meal_from_menu__already_removed() {
    let mut test_meal = rnd_meal();
    test_meal.removed = true;
    test_meal.remove_meal_from_menu();

    assert!(test_meal.removed);
    assert!(!test_meal.visible());

    let popped_events = test_meal.entity_params.pop_events();
    assert!(popped_events.is_empty());
}
