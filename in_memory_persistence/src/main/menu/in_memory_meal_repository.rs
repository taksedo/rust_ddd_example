use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use common::events::main::domain_event_publisher::DomainEventPublisher;
use common::types::main::base::domain_entity::DomainEntityTrait;
use derivative::Derivative;
use derive_new::new;

use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::MealEventEnum;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::meal_name::MealName;
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryMealRepository {
    pub event_publisher: Arc<Mutex<dyn DomainEventPublisher<MealEventEnum>>>,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<MealId, Meal>,
}

impl MealPersister for InMemoryMealRepository {
    fn save(&mut self, mut meal: Meal) {
        self.event_publisher
            .lock()
            .unwrap()
            .publish(&meal.entity_params.pop_events());
        self.storage.insert(meal.entity_params.id, meal);
    }
}

impl MealExtractor for InMemoryMealRepository {
    fn get_by_id(&mut self, id: MealId) -> Option<Meal> {
        self.storage.get(&id).map(|x| x.to_owned()).take()
    }

    fn get_by_name(&mut self, name: MealName) -> Option<Meal> {
        self.storage
            .values()
            .map(|value| value.to_owned())
            .find(|value| value.name == name)
    }

    fn get_all(&mut self) -> Vec<Meal> {
        let storage: &HashMap<MealId, Meal> = &self.storage;
        storage
            .iter()
            .filter(|(&_k, v)| !v.to_owned().removed)
            .map(|(&_k, v)| v.to_owned())
            .collect()
    }
}
