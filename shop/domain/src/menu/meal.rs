use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use common::types::{
    base::domain_entity::{DomainEntity, DomainEntityTrait, Version},
    errors::error::BusinessError,
};
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::menu::{
    meal_already_exists::MealAlreadyExists,
    meal_events::{MealAddedToMenuDomainEvent, MealEventEnum, MealRemovedFromMenuDomainEvent},
    value_objects::{
        meal_description::MealDescription,
        meal_id::{MealId, MealIdGenerator},
        meal_name::MealName,
        price::Price,
    },
};

#[derive(new, Debug, Clone, PartialEq, Default, Serialize, Deserialize, Getters)]
pub struct Meal {
    #[getter(skip)]
    entity_params: DomainEntity<MealId, MealEventEnum>,
    name: MealName,
    description: MealDescription,
    price: Price,
    #[new(value = "false")]
    removed: bool,
}

impl Meal {
    pub fn with_all_args(
        entity_params: DomainEntity<MealId, MealEventEnum>,
        name: MealName,
        description: MealDescription,
        price: Price,
        removed: bool,
    ) -> Self {
        Self {
            entity_params,
            name,
            description,
            price,
            removed,
        }
    }
    pub fn add_meal_to_menu(
        id_generator: Arc<Mutex<dyn MealIdGenerator>>,
        meal_exists: Arc<Mutex<dyn MealAlreadyExists>>,
        name: MealName,
        description: MealDescription,
        price: Price,
    ) -> Result<Meal, MealError> {
        if meal_exists.lock().unwrap().invoke(&name) {
            Err(MealError::AlreadyExistsWithSameNameError)
        } else {
            let id = id_generator.lock().unwrap().generate();

            //     .map_err(|_e: Error| MealError::IdGenerationError)?;
            let mut meal = Meal::new(
                DomainEntity::new(id, Version::default()),
                name,
                description,
                price,
            );
            meal.entity_params
                .add_event(MealAddedToMenuDomainEvent::new(id).into());
            Ok(meal)
        }
    }

    pub fn visible(&self) -> bool {
        !self.removed
    }

    pub fn remove_meal_from_menu(&mut self) {
        if !self.removed {
            self.removed = true;
            let id = self.entity_params.id;
            self.entity_params
                .add_event(MealRemovedFromMenuDomainEvent::new(id).into())
        }
    }

    pub fn id(&self) -> &MealId {
        self.entity_params.id()
    }

    pub fn version(&self) -> &Version {
        self.entity_params.version()
    }

    pub fn pop_events(&mut self) -> Vec<MealEventEnum> {
        self.entity_params.pop_events()
    }
}

#[derive(Debug, PartialEq)]
pub enum MealError {
    AlreadyExistsWithSameNameError,
}

impl BusinessError for MealError {}

#[allow(non_snake_case)]
#[cfg(all(test, feature = "domain"))]
mod tests {
    use std::sync::atomic::AtomicI64;

    use super::*;
    use crate::test_fixtures::{
        print_type_of, rnd_meal, rnd_meal_description, rnd_meal_id, rnd_meal_name, rnd_price,
        rnd_removed_meal,
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
            id_generator.clone(),
            meal_exists,
            name.to_owned(),
            description.to_owned(),
            price.to_owned(),
        );

        let mut test_meal = result.unwrap();
        assert_eq!(test_meal.id(), &id_generator.lock().unwrap().meal_id);
        assert_eq!(*test_meal.name(), name);
        assert_eq!(*test_meal.description(), description);
        assert_eq!(*test_meal.price(), price);
        assert!(test_meal.visible());

        let popped_events = test_meal.pop_events();
        let popped_events = popped_events.first().unwrap();

        let expected_event: &MealEventEnum =
            &MealAddedToMenuDomainEvent::new(id_generator.lock().unwrap().meal_id).into();
        assert_eq!(print_type_of(popped_events), print_type_of(expected_event));
    }

    #[test]
    fn add_meal_to_menu__already_exists_with_the_same_name() {
        let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new()));
        let meal_exists = Arc::new(Mutex::new(TestMealAlreadyExists { value: true }));
        let name = rnd_meal_name();
        let description = rnd_meal_description();
        let price = rnd_price();
        let result = Meal::add_meal_to_menu(id_generator, meal_exists, name, description, price);

        assert_eq!(
            result.unwrap_err(),
            MealError::AlreadyExistsWithSameNameError
        );
    }

    #[test]
    fn remove_meal_from_menu__success() {
        let mut test_meal = rnd_meal();
        test_meal.remove_meal_from_menu();
        assert!(test_meal.removed());
        assert!(!test_meal.visible());

        let popped_events = test_meal.pop_events();
        let popped_events = popped_events.get(0).unwrap();

        let expected_event = &MealEventEnum::MealRemovedFromMenuDomainEvent(
            MealRemovedFromMenuDomainEvent::new(*test_meal.id()),
        );
        assert_eq!(
            print_type_of(&popped_events),
            print_type_of(&expected_event)
        );
    }

    #[test]
    fn remove_meal_from_menu__already_removed() {
        let mut test_meal = rnd_removed_meal();
        test_meal.remove_meal_from_menu();

        assert!(test_meal.removed());
        assert!(!test_meal.visible());

        let popped_events = test_meal.pop_events();
        assert!(popped_events.is_empty());
    }
}
