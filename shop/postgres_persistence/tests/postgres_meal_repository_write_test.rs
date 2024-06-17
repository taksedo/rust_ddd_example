use std::sync::{Arc, Mutex};

use diesel::{sql_query, RunQueryDsl};
use diesel_migrations::MigrationHarness;
use domain::{
    main::menu::meal_events::{MealAddedToMenuDomainEvent, MealEventEnum},
    test_fixtures::{rnd_meal_id, rnd_meal_name},
};
use postgres_persistence::main::{
    database_start::MIGRATIONS, meal_db_dto::MealDbDto,
    postgres_meal_repository::PostgresMealRepository,
};
use usecase::main::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

use crate::test_fixtures::{rnd_meal_with_event, EntitySetters, MockEventPublisher, TestDb};

mod test_fixtures;

#[test]
fn save_new_instance() {
    let rnd_meal = rnd_meal_with_event(rnd_meal_id());

    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());
    repository.save(rnd_meal.clone());

    publisher
        .lock()
        .unwrap()
        .verify_contains(vec![
            MealAddedToMenuDomainEvent::new(*rnd_meal.get_id()).into()
        ]);

    let result = repository.get_all();
    dbg!(&result);
    assert!(!result.is_empty())
}

#[test]
#[should_panic]
fn save_new_instance_but_already_exists_with_the_same_id() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let meal_id = rnd_meal_id();
    let first = rnd_meal_with_event(meal_id);
    let second = rnd_meal_with_event(meal_id);
    let first = first.set_id(meal_id);
    second.set_id(meal_id);

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

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let meal_name = rnd_meal_name();
    let first = rnd_meal_with_event(rnd_meal_id());
    let second = rnd_meal_with_event(rnd_meal_id());
    let first = first.set_name(meal_name.clone());
    let second = second.set_name(meal_name);

    repository.save(first);
    repository.save(second);
}

#[test]
fn create_new_instance_and_then_update_it() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_meal_with_event(rnd_meal_id());
    let meal_id = *rnd_meal.clone().get_id();
    repository.save(rnd_meal);

    let mut rnd_meal = repository.get_by_id(&meal_id).unwrap();

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
    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_meal_with_event(rnd_meal_id());
    let meal_id = *rnd_meal.clone().get_id();
    repository.save(rnd_meal);

    let rnd_meal = repository.get_by_id(&meal_id).unwrap();

    repository.save(rnd_meal.clone());

    publisher
        .lock()
        .unwrap()
        .verify_contains(vec![Into::<MealEventEnum>::into(
            MealAddedToMenuDomainEvent::new(*rnd_meal.get_id()),
        )]);
}

#[should_panic]
#[test]
fn saving_failed_if_version_outdated() {
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = Arc::new(Mutex::new(MockEventPublisher::default()));
    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_meal_with_event(rnd_meal_id());
    repository.save(rnd_meal.clone());

    let mut copy_of_rnd_meal = rnd_meal;
    copy_of_rnd_meal.remove_meal_from_menu();

    repository.save(copy_of_rnd_meal);
}
