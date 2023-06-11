use crate::main::meal_db_dto::MealDbDto;
use common_events::main::domain_event_publisher::DomainEventPublisher;
use common_types::main::base::domain_entity::DomainEntityTrait;
use derivative::Derivative;
use derive_new::new;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::DomainEventEnum;
use domain::main::menu::meal_id::MealId;
use domain::main::menu::meal_name::MealName;
use std::sync::{Arc, Mutex};
use usecase::main::menu::access::meal_extractor::MealExtractor;
use usecase::main::menu::access::meal_persister::MealPersister;

#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct PostgresMealRepository {
    #[derivative(Debug = "ignore")]
    pub connection: PgConnection,
    pub event_publisher: Arc<Mutex<dyn DomainEventPublisher<DomainEventEnum>>>,
}

impl MealPersister for PostgresMealRepository {
    fn save(&mut self, mut meal: Meal) {
        let connection = &mut self.connection;
        let new_meal = MealDbDto::from(meal.clone());
        self.event_publisher
            .lock()
            .unwrap()
            .publish(&meal.pop_events());
        diesel::insert_into(crate::main::schema::shop::meal::dsl::meal)
            .values(&new_meal)
            .returning(MealDbDto::as_returning())
            .get_result(connection)
            .expect("Error saving new meal");
    }
}

impl MealExtractor for PostgresMealRepository {
    fn get_by_id(&mut self, meal_id: MealId) -> Option<Meal> {
        use super::schema::shop::meal::dsl::*;
        let connection = &mut self.connection;
        let result = meal
            .find(meal_id.to_i64())
            .select(MealDbDto::as_select())
            .load(connection);

        match result {
            Ok(meal_res) => {
                if !meal_res.is_empty() {
                    let res: Vec<Meal> = meal_res
                        .iter()
                        .map(|meal_res_iter| Meal::from(meal_res_iter.clone()))
                        .collect();
                    Some(res.get(0).unwrap().clone())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn get_by_name(&mut self, meal_name: MealName) -> Option<Meal> {
        use super::schema::shop::meal::dsl::*;
        let connection = &mut self.connection;

        let result = meal
            .filter(name.eq(meal_name.to_string()))
            .select(MealDbDto::as_select())
            .load(connection);

        match result {
            Ok(meal_res) => {
                if !meal_res.is_empty() {
                    let res: Vec<Meal> = meal_res
                        .iter()
                        .map(|meal_res_iter| Meal::from(meal_res_iter.clone()))
                        .collect();
                    Some(res.get(0).unwrap().clone())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn get_all(&mut self) -> Vec<Meal> {
        use super::schema::shop::meal::dsl::*;
        let connection = &mut self.connection;

        let result = meal.select(MealDbDto::as_select()).load(connection);

        result
            .unwrap()
            .iter()
            .filter(|meal_res_iter| !meal_res_iter.removed)
            .map(|meal_res_iter| Meal::from(meal_res_iter.clone()))
            .collect::<Vec<Meal>>()
    }
}
