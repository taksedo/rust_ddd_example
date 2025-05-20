use common::types::base::{AM, AMTrait};
use diesel::{RunQueryDsl, sql_query};
use diesel_migrations::MigrationHarness;
use domain::{
    menu::meal_events::{MealAddedToMenuDomainEvent, MealEventEnum},
    test_fixtures::*,
};
use postgres_persistence::{
    database_start::MIGRATIONS, meal_db_dto::MealDbDto,
    postgres_meal_repository::PostgresMealRepository,
};
use usecase::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

use crate::test_fixtures::{
    MockEventPublisher, TestDb, rnd_new_meal_with_meal_id, rnd_new_meal_with_name,
};

mod test_fixtures;

#[tokio::test]
async fn save_new_instance() {
    let rnd_meal = rnd_new_meal_with_meal_id(rnd_meal_id()).await;

    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());
    repository.save(rnd_meal.clone()).await;

    publisher
        .lock()
        .await
        .verify_contains(vec![MealAddedToMenuDomainEvent::new(*rnd_meal.id()).into()]);

    let result = repository.get_all();
    dbg!(&result);
    assert!(!result.is_empty())
}

#[tokio::test]
#[should_panic(
    expected = "Error saving new meal: DatabaseError(UniqueViolation, \"duplicate key value violates unique constraint \\\"meal_pkey\\\"\")"
)]
async fn save_new_instance_but_already_exists_with_the_same_id() {
    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let meal_id = rnd_meal_id();
    let first = rnd_new_meal_with_meal_id(meal_id).await;
    let second = rnd_new_meal_with_meal_id(meal_id).await;

    repository.save(first).await;
    repository.save(second).await;
}

#[tokio::test]
#[should_panic(
    expected = "Error saving new meal: DatabaseError(UniqueViolation, \"duplicate key value violates unique constraint \\\"meal_name_key\\\"\")"
)]
async fn save_new_instance_but_already_exists_with_the_same_name() {
    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let meal_name = rnd_meal_name();
    let first = rnd_new_meal_with_name(&meal_name).await;
    let second = rnd_new_meal_with_name(&meal_name).await;

    repository.save(first).await;
    repository.save(second).await;
}

#[tokio::test]
async fn create_new_instance_and_then_update_it() {
    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());

    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_new_meal_with_meal_id(rnd_meal_id()).await;
    let meal_id = *rnd_meal.clone().id();
    repository.save(rnd_meal).await;

    let mut rnd_meal = repository.get_by_id(&meal_id).unwrap();

    rnd_meal.remove_meal_from_menu();
    repository.save(rnd_meal.clone()).await;

    let mut conn = db.conn();
    let meal_in_db = sql_query("SELECT * FROM shop.meal")
        .load::<MealDbDto>(&mut conn)
        .unwrap();
    assert!(!meal_in_db.is_empty())
}

#[tokio::test]
async fn save_again_without_changes() {
    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());
    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_new_meal_with_meal_id(rnd_meal_id()).await;
    let meal_id = *rnd_meal.clone().id();
    repository.save(rnd_meal).await;

    let rnd_meal = repository.get_by_id(&meal_id).unwrap();

    repository.save(rnd_meal.clone()).await;

    publisher
        .lock()
        .await
        .verify_contains(vec![Into::<MealEventEnum>::into(
            MealAddedToMenuDomainEvent::new(*rnd_meal.id()),
        )]);
}

#[tokio::test]
#[should_panic(
    expected = "Error saving new meal: DatabaseError(UniqueViolation, \"duplicate key value violates unique constraint \\\"meal_pkey\\\"\")"
)]
async fn saving_failed_if_version_outdated() {
    let db = TestDb::new().await;
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let publisher = AM::new_am(MockEventPublisher::default());
    let mut repository = PostgresMealRepository::new(conn, publisher.clone());

    let rnd_meal = rnd_new_meal_with_meal_id(rnd_meal_id()).await;
    repository.save(rnd_meal.clone()).await;

    let mut copy_of_rnd_meal = rnd_meal;
    copy_of_rnd_meal.remove_meal_from_menu();

    repository.save(copy_of_rnd_meal).await;
}
