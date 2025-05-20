use async_trait::async_trait;
use common::{events::DomainEventPublisher, types::base::AM};
use derivative::Derivative;
use derive_new::new;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::menu::{
    meal::Meal,
    meal_events::MealEventEnum,
    value_objects::{meal_id::MealId, meal_name::MealName},
};
use usecase::menu::access::{meal_extractor::MealExtractor, meal_persister::MealPersister};

use crate::{meal_db_dto::MealDbDto, schema::shop::meal::dsl::*};

#[derive(Derivative, new)]
#[derivative(Debug)]
pub struct PostgresMealRepository {
    #[derivative(Debug = "ignore")]
    pub connection: PgConnection,
    pub event_publisher: AM<dyn DomainEventPublisher<MealEventEnum>>,
}

impl PostgresMealRepository {
    fn update(&mut self, meal_param: Meal) {
        let connection = &mut self.connection;
        let new_meal = MealDbDto::from(meal_param.clone());
        let meal_id = meal_param.id().to_i64();
        let previous_version = meal_param.version().previous().to_i64();

        diesel::update(meal)
            .filter(id.eq(meal_id))
            .filter(version.eq(previous_version))
            .set(&new_meal)
            .execute(connection)
            .unwrap_or_else(|_| {
                panic!(
                    "Meal #{} [version = {}] is outdated",
                    meal_id,
                    meal_param.version().to_i64()
                )
            });
    }

    fn insert(&mut self, meal_param: Meal) {
        let connection = &mut self.connection;
        let new_meal = MealDbDto::from(meal_param);
        diesel::insert_into(meal)
            .values(&new_meal)
            .returning(MealDbDto::as_returning())
            .get_result(connection)
            .expect("Error saving new meal");
    }
}

#[async_trait]
impl MealPersister for PostgresMealRepository {
    async fn save(&mut self, mut meal_param: Meal) {
        let events = meal_param.pop_events();
        if !events.is_empty() {
            let mut flag = false;
            for event in events.iter() {
                if matches!(event, MealEventEnum::MealAddedToMenuDomainEvent(x) if &x.meal_id == meal_param.id())
                {
                    self.insert(meal_param.clone());
                    flag = true;
                    break;
                }
            }
            if !flag {
                self.update(meal_param);
            }
            self.event_publisher.lock().await.publish(&events).await;
        }
    }
}

impl MealExtractor for PostgresMealRepository {
    fn get_by_id(&mut self, meal_id: &MealId) -> Option<Meal> {
        use super::schema::shop::meal::dsl::*;
        let connection = &mut self.connection;
        let result = meal
            .find(meal_id.to_i64())
            .select(MealDbDto::as_select())
            .get_result(connection)
            .ok()?;

        Some(Meal::from(result))
    }

    fn get_by_name(&mut self, meal_name: &MealName) -> Option<Meal> {
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
                    Some(res.first().unwrap().clone())
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
