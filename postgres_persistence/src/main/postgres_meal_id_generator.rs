use crate::main::meal_db_dto::establish_connection;
use derive_new::new;
use diesel::sql_types;
use diesel::{select, sql_function, RunQueryDsl};
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::fmt::Debug;

#[derive(new, Debug)]
pub struct PostgresMealIdGenerator {}

impl MealIdGenerator for PostgresMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let connection = &mut establish_connection();
        let id = select(nextval("shop.meal_id_seq"))
            .get_result::<i64>(connection)
            .unwrap();

        MealId::new(id)
    }
}

sql_function!(fn nextval(a: sql_types::Text) -> sql_types::BigInt);
