use derivative::Derivative;
use derive_new::new;
use diesel::{select, sql_function, RunQueryDsl};
use diesel::{sql_types, PgConnection};
use domain::main::menu::meal_id::{MealId, MealIdGenerator};
use std::fmt::Debug;

#[derive(new, Derivative)]
#[derivative(Debug)]
pub struct PostgresMealIdGenerator {
    #[derivative(Debug = "ignore")]
    connection: PgConnection,
}

impl MealIdGenerator for PostgresMealIdGenerator {
    fn generate(&mut self) -> MealId {
        let connection = &mut self.connection;
        let id = select(nextval("shop.meal_id_seq"))
            .get_result::<i64>(connection)
            .unwrap();

        MealId::new(id)
    }
}

sql_function!(fn nextval(a: sql_types::Text) -> sql_types::BigInt);
