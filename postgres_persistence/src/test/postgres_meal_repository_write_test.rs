use crate::main::database_start::MIGRATIONS;
use crate::main::meal_db_dto::MealDbDto;
use crate::main::postgres_meal_repository::PostgresMealRepository;
use crate::test_fixtures::{rnd_meal_with_event, MockEventPublisher, TestDb};
use diesel::{sql_query, RunQueryDsl};
use diesel_migrations::MigrationHarness;
use domain::main::menu::meal_events::{DomainEventEnum, MealAddedToMenuDomainEvent};
use domain::test_fixtures::fixtures::{rnd_meal_id, rnd_meal_name};
use std::sync::{Arc, Mutex};
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[test]
fn save_new_instance() {
    let rnd_meal = rnd_meal_with_event();

    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);
    repository.save(rnd_meal.clone());

    publisher
        .lock()
        .unwrap()
        .verify_contains(vec![DomainEventEnum::MealAddedToMenuDomainEvent(
            MealAddedToMenuDomainEvent::new(rnd_meal.domain_entity_field.id),
        )]);

    let result = repository.get_all();
    assert!(!result.is_empty())
}

#[test]
// #[should_panic]
fn save_new_instance_but_already_exists_with_the_same_id() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);

    let meal_id = rnd_meal_id();
    let mut first = rnd_meal_with_event();
    let mut second = rnd_meal_with_event();
    first.domain_entity_field.id = meal_id;
    second.domain_entity_field.id = meal_id;

    repository.save(first);
    repository.save(second);
}

#[test]
#[should_panic]
fn save_new_instance_but_already_exists_with_the_same_name() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);

    let meal_name = rnd_meal_name();
    let mut first = rnd_meal_with_event();
    let mut second = rnd_meal_with_event();
    first.name = meal_name.clone();
    second.name = meal_name.clone();

    repository.save(first);
    repository.save(second);
}

#[test]
fn create_new_instance_and_then_update_it() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);

    let rnd_meal = rnd_meal_with_event();
    let meal_id = rnd_meal.domain_entity_field.id;
    repository.save(rnd_meal);

    let mut rnd_meal = repository.get_by_id(meal_id).unwrap();

    rnd_meal.remove_meal_from_menu();
    repository.save(rnd_meal.clone());

    let mut conn = db.conn();
    let meal_in_db = sql_query("SELECT * FROM shop.meal")
        .load::<MealDbDto>(&mut conn)
        .unwrap();
    assert!(!meal_in_db.is_empty())
}

#[test]
fn save_again_without_changes() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));
    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);

    let rnd_meal = rnd_meal_with_event();
    let meal_id = rnd_meal.domain_entity_field.id;
    repository.save(rnd_meal);

    let rnd_meal = repository.get_by_id(meal_id).unwrap();

    repository.save(rnd_meal.clone());

    publisher
        .lock()
        .unwrap()
        .verify_contains(vec![Into::<DomainEventEnum>::into(
            MealAddedToMenuDomainEvent::new(rnd_meal.domain_entity_field.id),
        )]);
}

#[should_panic]
#[test]
fn saving_failed_if_version_outdated() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));
    let mut repository = PostgresMealRepository::new(conn, Arc::clone(&publisher) as _);

    let rnd_meal = rnd_meal_with_event();
    repository.save(rnd_meal.clone());

    let mut copy_of_rnd_meal = rnd_meal.clone();
    copy_of_rnd_meal.remove_meal_from_menu();

    repository.save(copy_of_rnd_meal);
}
