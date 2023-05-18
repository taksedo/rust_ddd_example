use crate::main::menu::access::meal_extractor::MealExtractor;
use crate::main::menu::access::meal_persister::MealPersister;
use common_types::main::base::domain_event::DomainEventTrait;
use derive_new::new;
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use domain::test_fixtures::fixtures::rnd_meal;
use std::collections::HashMap;

pub fn removed_meal() -> Meal {
    let mut meal = rnd_meal();
    meal.remove_meal_from_menu();
    meal
}
#[derive(new, Debug, Clone)]
pub struct TestEvent {}

impl DomainEventTrait for TestEvent {}

#[derive(new, Debug, Clone)]
pub struct MockMealPersister {
    #[new(value = "None")]
    pub meal: Option<Meal>,
}

impl MockMealPersister {
    pub fn verify_invoked(
        &self,
        id: Option<MealId>,
        name: Option<MealName>,
        // description: Option<MealDescription>,
        // price: Option<Price>,
    ) {
        if id.is_some() {
            assert_eq!(
                self.to_owned().meal.unwrap().domain_entity_field.id,
                id.unwrap()
            )
        }
        if name.is_some() {
            assert_eq!(self.to_owned().meal.unwrap().name, name.unwrap())
        }
        // if description.is_some() {
        //     assert_eq!(
        //         self.to_owned().meal.unwrap().description,
        //         description.unwrap()
        //     )
        // }
        // if price.is_some() {
        //     assert_eq!(self.to_owned().meal.unwrap().price, price.unwrap())
        // }
    }
    pub fn verify_invoked_meal(&self, meal: Option<Meal>) {
        if meal.is_some() {
            assert_eq!(self.to_owned().meal, meal)
        }
    }
    // pub fn verify_events_after_deletion(&mut self, id: MealId) {
    //     let event_enum: DomainEventEnum = MealRemovedFromMenuDomainEvent::new(id).into();
    //     assert_eq!(self.to_owned().meal.unwrap().pop_events(), vec![event_enum]);
    // }
    pub fn verify_empty(&self) {
        assert!(&self.meal.is_none());
    }
}

impl MealPersister for MockMealPersister {
    fn save(&mut self, meal: Meal) {
        self.meal = Some(meal);
    }
}

#[derive(new, Clone, PartialEq, Debug)]
pub struct TestMealExtractor {
    #[new(value = "HashMap::new()")]
    pub value: HashMap<MealId, Meal>,
}

impl MealExtractor for TestMealExtractor {
    fn get_by_id(&mut self, id: MealId) -> Option<&Meal> {
        self.value.get(&id)
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
        let result = self
            .clone()
            .value
            .iter()
            .find_map(|(key, val)| if val.name == name { Some(key) } else { None })
            .and_then(|meal_id| self.get_by_id(*meal_id).cloned());
        result
    }

    fn get_all(&mut self) -> Vec<Meal> {
        self.value.clone().into_values().collect()
    }
}
