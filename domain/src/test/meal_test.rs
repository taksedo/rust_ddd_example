use crate::main::menu::meal::Meal;
use crate::main::menu::meal::MealError::AlreadyExistsWithSameNameError;
use crate::main::menu::meal_already_exists::MealAlreadyExists;
use crate::main::menu::meal_events::{MealAddedToMenuDomainEvent, MealRemovedFromMenuDomainEvent};
use crate::main::menu::meal_id::{MealId, MealIdGenerator};
use crate::main::menu::meal_name::MealName;
use crate::test_fixtures::fixtures::{print_type_of, rnd_meal, rnd_meal_id, rnd_meal_name};
use common_types::main::base::domain_entity::DomainEntityTrait;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::atomic::AtomicI64;

#[derive(Debug, new, Default)]
pub(crate) struct TestMealIdGenerator {
    #[new(value = "AtomicI64::from(0)")]
    _counter: AtomicI64,
    #[new(value = "rnd_meal_id()")]
    pub meal_id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&self) -> MealId {
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
#[allow(non_snake_case)]
fn add_meal__success() {
    let id_generator = Rc::new(TestMealIdGenerator::new());
    let meal_exists = Rc::new(RefCell::new(TestMealAlreadyExists { value: false }));
    let name = rnd_meal_name();
    // let description = rnd_meal_description();
    // let price = rnd_price();
    let result = Meal::add_meal_to_menu(
        id_generator.to_owned(),
        meal_exists,
        name.to_owned(),
        // description.to_owned(),
        // price.to_owned(),
    );

    let test_meal = result.unwrap();
    assert_eq!(test_meal.domain_entity_field.id, id_generator.meal_id);
    assert_eq!(test_meal.name, name);
    assert!(test_meal.visible());

    let popped_events = test_meal.pop_events().to_owned();
    let received_event = popped_events.get(0).unwrap().borrow();

    let expected_event_binding =
        RefCell::new(MealAddedToMenuDomainEvent::new(id_generator.meal_id));
    let expected_event: Ref<dyn DomainEventTrait> = expected_event_binding.borrow();
    assert_eq!(
        print_type_of(&received_event),
        print_type_of(&expected_event)
    );
}

#[test]
#[allow(non_snake_case)]
fn add_meal_to_menu__already_exists_with_the_same_name() {
    let id_generator = Rc::new(TestMealIdGenerator::new());
    let meal_exists = Rc::new(RefCell::new(TestMealAlreadyExists { value: true }));
    let name = rnd_meal_name();
    let result = Meal::add_meal_to_menu(
        id_generator,
        meal_exists,
        name,
        // description.to_owned(),
        // price.to_owned(),
    );

    assert_eq!(result.unwrap_err(), AlreadyExistsWithSameNameError);
}

#[test]
#[allow(non_snake_case)]
fn remove_meal_from_menu__success() {
    let mut test_meal = rnd_meal();
    test_meal.remove_meal_from_menu();
    assert!(test_meal.removed);
    assert!(!test_meal.visible());

    let popped_events_binding = test_meal.pop_events().to_owned();
    let popped_events = popped_events_binding.get(0).unwrap().borrow();

    let expected_event_binding = RefCell::new(MealRemovedFromMenuDomainEvent::new(
        test_meal.domain_entity_field.id,
    ));
    let expected_event: Ref<dyn DomainEventTrait> = expected_event_binding.borrow();
    assert_eq!(
        print_type_of(&popped_events),
        print_type_of(&expected_event)
    );
}

#[test]
#[allow(non_snake_case)]
fn remove_meal_from_menu__already_removed() {
    let mut test_meal = rnd_meal();
    test_meal.removed = true;
    test_meal.remove_meal_from_menu();

    assert!(test_meal.removed);
    assert!(!test_meal.visible());

    let popped_events = test_meal.pop_events().to_owned();
    assert_eq!(popped_events.len(), 0);
}
