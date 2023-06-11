use crate::main::database::MIGRATIONS;
use crate::main::postgres_meal_id_generator::PostgresMealIdGenerator;
use crate::test_fixtures::TestDb;
use diesel::sql_types::{BigInt, VarChar};
use diesel::{select, sql_function, RunQueryDsl};
use diesel_migrations::MigrationHarness;
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::rnd_meal_id;
use sea_orm::{DatabaseBackend, MockDatabase, Transaction};
use std::fmt::format;

#[test]
fn generate_id_integration_test() {
    let rnd_id = rnd_meal_id();
    let db = TestDb::new();
    let mut conn = db.conn();

    conn.run_pending_migrations(MIGRATIONS).unwrap();

    let conn = db.conn();
    let mut id_generator = PostgresMealIdGenerator::new(conn);

    let mut conn = db.conn();
    select(setval("shop.meal_id_seq", rnd_id.to_i64()))
        .execute(&mut conn)
        .unwrap();
    let meal_id = id_generator.generate();

    assert_eq!(meal_id, MealId::new(rnd_id.to_i64() + 1));
}

sql_function!(fn setval(x: VarChar, y: BigInt));
