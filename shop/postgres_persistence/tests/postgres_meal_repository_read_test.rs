#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use diesel_migrations::MigrationHarness;
use domain_test_fixtures::{rnd_meal_id, rnd_meal_name};
use postgres_persistence::{
    database_start::MIGRATIONS, postgres_meal_repository::PostgresMealRepository,
};
use postgres_persistence_test_fixtures::{rnd_new_meal_with_meal_id, MockEventPublisher, TestDb};
use usecase::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

#[test]
fn get_by_id__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));

    let result = repository.get_by_id(&rnd_meal_id());

    assert!(result.is_none())
}

#[test]
fn get_by_id__successfully_returned() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let meal_id = *meal.id();
    let result = repository.get_by_id(&meal_id);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
fn get_by_name__not_found() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));

    let result = repository.get_by_name(&rnd_meal_name());

    assert!(result.is_none());
}

#[test]
fn get_by_name__successfully_returned() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let mut repository =
        PostgresMealRepository::new(conn, Arc::new(Mutex::new(MockEventPublisher::default())));
    repository.save(meal.clone());

    let meal_name = meal.name();
    let result = repository.get_by_name(meal_name);

    assert!(result.is_some());
    assert_eq!(result.unwrap(), meal);
}

#[test]
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
fn get_all__table_is_not_empty() {
    let meal = rnd_new_meal_with_meal_id(rnd_meal_id());
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
fn get_all__table_is_not_empty_but_removed() {
    let mut meal = rnd_new_meal_with_meal_id(rnd_meal_id());
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
