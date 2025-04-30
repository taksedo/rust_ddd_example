use std::fmt::Debug;

use derivative::Derivative;
use derive_new::new;
use diesel::{PgConnection, RunQueryDsl, define_sql_function, select, sql_types::Text};
use domain::menu::value_objects::meal_id::{MealId, MealIdGenerator};

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

        MealId::try_from(id).unwrap()
    }
}

define_sql_function!(fn nextval(a: Text) -> BigInt);
