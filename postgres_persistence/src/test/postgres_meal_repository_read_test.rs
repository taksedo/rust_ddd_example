use crate::main::database_start::MIGRATIONS;
use crate::main::postgres_meal_repository::PostgresMealRepository;
use crate::test_fixtures::{rnd_meal_with_event, MockEventPublisher, TestDb};
use diesel_migrations::MigrationHarness;
use domain::test_fixtures::fixtures::{rnd_meal_id, rnd_meal_name};
use std::sync::{Arc, Mutex};
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[test]
#[allow(non_snake_case)]
fn get_by_id__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));

    let result = repository.get_by_id(rnd_meal_id());

    assert!(result.is_none())
}

#[test]
#[allow(non_snake_case)]
fn get_by_id__successfully_returned() {
    let meal = rnd_meal_with_event(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let meal_id = meal.entity_params.id;
    let result = repository.get_by_id(meal_id);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
#[allow(non_snake_case)]
fn get_by_name__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));

    let result = repository.get_by_name(rnd_meal_name());

    assert!(result.is_none());
}

#[test]
#[allow(non_snake_case)]
fn get_by_name__successfully_returned() {
    let meal = rnd_meal_with_event(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let meal_name = meal.name.clone();
    let result = repository.get_by_name(meal_name);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
#[allow(non_snake_case)]
fn get_all__table_is_empty() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));

    let result = repository.get_all();

    assert!(result.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn get_all__table_is_not_empty() {
    let meal = rnd_meal_with_event(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let result = repository.get_all();

    assert!(!result.is_empty());
    assert_eq!(result.get(0).unwrap(), &meal);
}

#[test]
#[allow(non_snake_case)]
fn get_all__table_is_not_empty_but_removed() {
    let mut meal = rnd_meal_with_event(rnd_meal_id());
    meal.remove_meal_from_menu();
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let result = repository.get_all();

    assert!(result.is_empty());
}
