use diesel::{
    select, sql_function,
    sql_types::{BigInt, VarChar},
    RunQueryDsl,
};
use diesel_migrations::MigrationHarness;
use domain::{
    main::menu::value_objects::meal_id::{MealId, MealIdGenerator},
    test_fixtures::rnd_meal_id,
};

use crate::{
    main::{database_start::MIGRATIONS, postgres_meal_id_generator::PostgresMealIdGenerator},
    test_fixtures::TestDb,
};

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

    assert_eq!(meal_id, MealId::try_from(rnd_id.to_i64() + 1).unwrap());
}

sql_function!(fn setval(x: VarChar, y: BigInt));
