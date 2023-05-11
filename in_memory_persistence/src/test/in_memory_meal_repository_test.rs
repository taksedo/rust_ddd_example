use crate::main::menu::in_memory_meal_repository::InMemoryMealRepository;
use crate::test_fixtures::fixtures::{meal_with_events, type_of, TestEventPublisher};
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::{MealAddedToMenuDomainEvent, MealRemovedFromMenuDomainEvent};
use domain::test_fixtures::fixtures::{rnd_meal, rnd_meal_id, rnd_meal_name};
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[allow(non_snake_case)]
#[test]
fn saving_meal__meal_doesnt_exist() {
    type E = MealRemovedFromMenuDomainEvent;
    let event_publisher = TestEventPublisher::<E>::new();
    let mut meal_repository = InMemoryMealRepository::new(event_publisher);
    let meal = meal_with_events();

    meal_repository.save(meal.clone());

    let stored_meal = meal_repository.storage.get(&meal.id).unwrap();
    assert_eq!(&meal, stored_meal);

    let event_publisher = meal_repository.event_publisher;
    assert_eq!(&event_publisher.storage.len(), &1);

    let event: &MealRemovedFromMenuDomainEvent = event_publisher.storage.get(0).unwrap();
    assert_eq!(event.meal_id, meal.id);
}

#[test]
#[allow(non_snake_case)]
fn saving_meal__meal_exists() {
    let existing_meal = rnd_meal();

    let event_publisher = TestEventPublisher::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    repository.storage.insert(existing_meal.id, existing_meal);

    let updated_meal = meal_with_events();
    repository.save(updated_meal.clone());

    let event = repository.event_publisher.storage.get(0).unwrap();
    assert_eq!(
        type_of(event),
        "&domain::main::menu::meal_events::MealRemovedFromMenuDomainEvent"
    );
    assert_eq!(event.meal_id, updated_meal.id);
}

#[test]
#[allow(non_snake_case)]
fn get_by_id__meal_exists() {
    let existing_meal: Meal<MealRemovedFromMenuDomainEvent> = rnd_meal();

    let event_publisher = TestEventPublisher::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    repository
        .storage
        .insert(existing_meal.id, existing_meal.clone());

    let meal = repository.get_by_id(existing_meal.id.to_owned()).unwrap();
    assert_eq!(type_of(meal), type_of(&existing_meal));
}

#[test]
#[allow(non_snake_case)]
fn get_by_id__meal_doesnt_exist() {
    let event_publisher = TestEventPublisher::<MealRemovedFromMenuDomainEvent>::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    let meal = repository.get_by_id(rnd_meal_id());
    assert!(meal.is_none());
}

#[test]
#[allow(non_snake_case)]
fn get_by_name__repository_is_empty() {
    let event_publisher = TestEventPublisher::<MealRemovedFromMenuDomainEvent>::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    let meal = repository.get_by_name(rnd_meal_name());
    assert!(meal.is_none());
}

#[test]
#[allow(non_snake_case)]
fn get_meal_by_name__success() {
    let stored_meal: Meal<MealRemovedFromMenuDomainEvent> = rnd_meal();
    let event_publisher = TestEventPublisher::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    repository.save(stored_meal.clone());

    let meal = repository.get_by_name(stored_meal.clone().name).unwrap();
    assert_eq!(type_of(meal), type_of(stored_meal));
}

#[test]
#[allow(non_snake_case)]
fn get_all_meals__repository_is_empty() {
    let event_publisher = TestEventPublisher::<MealRemovedFromMenuDomainEvent>::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    let meals = repository.get_all();
    assert!(meals.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn get_all_meals__success() {
    let event_publisher = TestEventPublisher::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    let stored_meal: Meal<MealAddedToMenuDomainEvent> = rnd_meal();
    repository
        .storage
        .insert(stored_meal.id, stored_meal.clone());

    let meals = repository.get_all();
    assert_eq!(meals.get(0).unwrap(), &stored_meal);
}

#[test]
#[allow(non_snake_case)]
fn get_all_meals__removed_is_not_returned() {
    let event_publisher = TestEventPublisher::<MealRemovedFromMenuDomainEvent>::new();
    let mut repository = InMemoryMealRepository::new(event_publisher);
    let mut stored_meal = rnd_meal();
    stored_meal.removed = true;
    repository.storage.insert(stored_meal.id, stored_meal);

    let meals = repository.get_all();
    assert!(meals.is_empty());
}
