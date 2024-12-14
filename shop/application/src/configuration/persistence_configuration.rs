use common::types::base::{AM, AMW};
use in_memory_persistence::order::{
    in_memory_incremental_shop_order_id_generator::InMemoryIncrementalShopOrderIdGenerator,
    in_memory_shop_order_repository::InMemoryShopOrderRepository,
};
use lazy_static::lazy_static;
use postgres_persistence::{
    database_start::establish_connection, postgres_meal_id_generator::PostgresMealIdGenerator,
    postgres_meal_repository::PostgresMealRepository,
};

use crate::configuration::application_configuration::EVENT_PUBLISHER;

pub type ORepository = OrderRepository;
type OrderRepository = InMemoryShopOrderRepository;
type OrderIdGenerator = InMemoryIncrementalShopOrderIdGenerator;
type MealIdGenerator = PostgresMealIdGenerator;
type MealRepository = PostgresMealRepository;

lazy_static! {
    pub(super) static ref MEAL_ID_GENERATOR: AM<MealIdGenerator> = meal_id_generator();
    /// `MealRepository` dependency injection
    pub(super) static ref MEAL_REPOSITORY: AM<MealRepository> = meal_repository();

    pub(super) static ref ORDER_ID_GENERATOR: AM<OrderIdGenerator> = order_id_generator();
    pub(super) static ref ORDER_REPOSITORY: AM<OrderRepository> = order_repository();
}

fn meal_id_generator() -> AM<MealIdGenerator> {
    AMW::new(MealIdGenerator::new(establish_connection()))
}

fn meal_repository() -> AM<MealRepository> {
    AMW::new(MealRepository::new(
        establish_connection(),
        EVENT_PUBLISHER.clone(),
    ))
}

fn order_id_generator() -> AM<OrderIdGenerator> {
    AMW::new(OrderIdGenerator::new())
}

pub fn order_repository() -> AM<ORepository> {
    AMW::new(ORepository::new(EVENT_PUBLISHER.clone()))
}
