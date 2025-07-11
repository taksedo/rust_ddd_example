use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use common::{events::DomainEventPublisher, types::base::AM};
use derivative::Derivative;
use derive_new::new;
use domain::menu::{
    meal::Meal,
    meal_events::MealEventEnum,
    value_objects::{meal_id::MealId, meal_name::MealName},
};
use usecase::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

#[derive(new, Clone, Derivative, Debug)]
pub struct InMemoryMealRepository {
    pub event_publisher: AM<dyn DomainEventPublisher<MealEventEnum>>,
    #[new(value = "HashMap::new()")]
    pub storage: HashMap<MealId, Meal>,
}

#[async_trait]
impl MealPersister for InMemoryMealRepository {
    async fn save(&mut self, mut meal: Meal) {
        self.event_publisher
            .lock()
            .await
            .publish(&meal.pop_events())
            .await;
        self.storage.insert(*meal.id(), meal);
    }
}

impl MealExtractor for InMemoryMealRepository {
    fn get_by_id(&mut self, id: &MealId) -> Option<Meal> {
        self.storage.get(id).map(|meal| meal.to_owned())
    }

    fn get_by_name(&mut self, name: &MealName) -> Option<Meal> {
        self.storage
            .values()
            .map(|value| value.to_owned())
            .find(|value| value.name() == name)
    }

    fn get_all(&mut self) -> Vec<Meal> {
        let storage: &HashMap<MealId, Meal> = &self.storage;
        storage
            .iter()
            .filter(|&(&_k, ref v)| !v.to_owned().removed())
            .map(|(&_k, v)| v.to_owned())
            .collect()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::any::{type_name, type_name_of_val};

    use common::types::base::AMTrait;
    use domain::{menu::meal_events::MealRemovedFromMenuDomainEvent, test_fixtures::*};

    use super::*;
    use crate::test_fixtures::*;

    #[tokio::test]
    async fn saving_meal__meal_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let storage_binding = event_publisher.clone();
        let mut meal_repository = InMemoryMealRepository::new(event_publisher);
        let meal = meal_with_events();

        meal_repository.save(meal.clone()).await;

        let stored_meal = meal_repository.storage.get(meal.id()).unwrap();
        assert_eq!(&meal, stored_meal);

        let storage = &storage_binding.lock().await.storage;
        assert_eq!(storage.len(), 1);

        let event: MealRemovedFromMenuDomainEvent =
            storage.first().unwrap().to_owned().try_into().unwrap();
        assert_eq!(event.meal_id, *meal.id());
    }

    #[tokio::test]
    async fn saving_meal__meal_exists() {
        let existing_meal = rnd_meal();

        let event_publisher = AM::new_am(TestEventPublisher::new());
        let storage_binding = event_publisher.clone();
        let mut meal_repository = InMemoryMealRepository::new(event_publisher);
        meal_repository
            .storage
            .insert(*existing_meal.id(), existing_meal);

        let updated_meal = meal_with_events();
        meal_repository.save(updated_meal.clone()).await;

        let storage = &storage_binding.lock().await.storage;
        let event = storage.first().unwrap().to_owned();
        let event: MealRemovedFromMenuDomainEvent = event.try_into().unwrap();
        assert_eq!(
            type_name_of_val(&event),
            type_name::<MealRemovedFromMenuDomainEvent>()
        );
        assert_eq!(event.meal_id, *updated_meal.id());
    }

    #[test]
    fn get_by_id__meal_exists() {
        let existing_meal = rnd_meal();
        let event_publisher = AM::new_am(TestEventPublisher::new());

        let mut meal_repository = InMemoryMealRepository::new(event_publisher);
        meal_repository
            .storage
            .insert(*existing_meal.id(), existing_meal.clone());

        let meal = meal_repository.get_by_id(existing_meal.id()).unwrap();
        assert_eq!(type_name_of_val(&meal), type_name_of_val(&existing_meal));
    }

    #[test]
    fn get_by_id__meal_doesnt_exist() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        let meal = repository.get_by_id(&rnd_meal_id());
        assert!(meal.is_none());
    }

    #[test]
    fn get_by_name__repository_is_empty() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        let meal = repository.get_by_name(&rnd_meal_name());
        assert!(meal.is_none());
    }

    #[tokio::test]
    async fn get_meal_by_name__success() {
        let stored_meal = rnd_meal();
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        repository.save(stored_meal.clone()).await;

        let meal = repository.get_by_name(stored_meal.clone().name()).unwrap();
        assert_eq!(type_name_of_val(&meal), type_name_of_val(&stored_meal));
    }

    #[test]
    fn get_all_meals__repository_is_empty() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        let meals = repository.get_all();
        assert!(meals.is_empty());
    }

    #[test]
    fn get_all_meals__success() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        let stored_meal = rnd_meal();
        repository
            .storage
            .insert(*stored_meal.id(), stored_meal.clone());

        let meals = repository.get_all();
        assert_eq!(meals.first().unwrap(), &stored_meal);
    }

    #[test]
    fn get_all_meals__removed_is_not_returned() {
        let event_publisher = AM::new_am(TestEventPublisher::new());
        let mut repository = InMemoryMealRepository::new(event_publisher);
        let stored_meal = rnd_removed_meal();
        repository.storage.insert(*stored_meal.id(), stored_meal);

        let meals = repository.get_all();
        assert!(meals.is_empty());
    }
}
