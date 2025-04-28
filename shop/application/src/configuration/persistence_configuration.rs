use std::sync::LazyLock;

use common::types::base::{AM, AMTrait};
use in_memory_persistence::order::{
    in_memory_incremental_shop_order_id_generator::InMemoryIncrementalShopOrderIdGenerator,
    in_memory_shop_order_repository::InMemoryShopOrderRepository,
};
use postgres_persistence::{
    database_start::establish_connection, postgres_meal_id_generator::PostgresMealIdGenerator,
    postgres_meal_repository::PostgresMealRepository,
};

use crate::configuration::application_configuration::EVENT_PUBLISHER;

pub type ORepository = OrderRepository;
type OrderRepository = InMemoryShopOrderRepository;
#[allow(dead_code)]
type OrderIdGenerator = InMemoryIncrementalShopOrderIdGenerator;
type MealIdGenerator = PostgresMealIdGenerator;
type MealRepository = PostgresMealRepository;

pub(super) static MEAL_ID_GENERATOR: LazyLock<AM<MealIdGenerator>> =
    LazyLock::new(meal_id_generator);
/// `MealRepository` dependency injection
pub(super) static MEAL_REPOSITORY: LazyLock<AM<MealRepository>> = LazyLock::new(meal_repository);

#[allow(dead_code)]
pub(super) static ORDER_ID_GENERATOR: LazyLock<AM<OrderIdGenerator>> =
    LazyLock::new(order_id_generator);
pub(super) static ORDER_REPOSITORY: LazyLock<AM<OrderRepository>> = LazyLock::new(order_repository);

fn meal_id_generator() -> AM<MealIdGenerator> {
    AM::new_am(MealIdGenerator::new(establish_connection()))
}

fn meal_repository() -> AM<MealRepository> {
    AM::new_am(MealRepository::new(
        establish_connection(),
        EVENT_PUBLISHER.clone(),
    ))
}

fn order_id_generator() -> AM<OrderIdGenerator> {
    AM::new_am(OrderIdGenerator::new())
}

pub fn order_repository() -> AM<ORepository> {
    AM::new_am(ORepository::new(EVENT_PUBLISHER.clone()))
}
