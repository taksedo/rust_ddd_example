use std::sync::{Arc, Mutex};

use common::types::main::base::generic_types::AM;
use in_memory_persistence::main::order::{
    in_memory_incremental_shop_order_id_generator::InMemoryIncrementalShopOrderIdGenerator as OrderIdGenerator,
    in_memory_shop_order_repository::InMemoryShopOrderRepository as OrderRepository,
};
use lazy_static::lazy_static;
use postgres_persistence::main::{
    database_start::establish_connection,
    postgres_meal_id_generator::PostgresMealIdGenerator as MealIdGenerator,
    postgres_meal_repository::PostgresMealRepository as MealRepository,
};

use crate::main::configuration::application_configuration::EVENT_PUBLISHER;

pub type OrderRepositoryType = OrderRepository;

lazy_static! {
    pub static ref MEAL_ID_GENERATOR: Arc<Mutex<MealIdGenerator>> = meal_id_generator();
    pub static ref MEAL_REPOSITORY: Arc<Mutex<MealRepository>> = meal_repository();
    pub static ref ORDER_ID_GENERATOR: Arc<Mutex<OrderIdGenerator>> = order_id_generator();
    pub static ref ORDER_REPOSITORY: AM<OrderRepository> = order_repository();
}

pub fn meal_id_generator() -> Arc<Mutex<MealIdGenerator>> {
    Arc::new(Mutex::new(MealIdGenerator::new(establish_connection())))
}

pub fn meal_repository() -> Arc<Mutex<MealRepository>> {
    Arc::new(Mutex::new(MealRepository::new(
        establish_connection(),
        EVENT_PUBLISHER.clone() as _,
    )))
}

pub fn order_id_generator() -> Arc<Mutex<OrderIdGenerator>> {
    Arc::new(Mutex::new(OrderIdGenerator::new()))
}

pub fn order_repository() -> AM<OrderRepositoryType> {
    Arc::new(Mutex::new(OrderRepositoryType::new(
        EVENT_PUBLISHER.clone() as _,
    )))
}
