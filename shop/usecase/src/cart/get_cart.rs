use std::fmt::Debug;

use async_trait::async_trait;
use common::types::common::Count;
use derive_new::new;
use domain::{
    cart::value_objects::customer_id::CustomerId,
    menu::value_objects::{meal_id::MealId, meal_name::MealName},
};
use thiserror::Error;

#[async_trait]
pub trait GetCart: Debug + Send {
    async fn execute(&self, for_customer: CustomerId) -> Result<CartInfo, GetCartUseCaseError>;
}

#[derive(new, Debug)]
pub struct CartInfo {
    pub for_customer: CustomerId,
    pub items: Vec<CartItem>,
}

#[derive(new, PartialEq, Debug)]
pub struct CartItem {
    pub meal_id: MealId,
    pub meal_name: MealName,
    pub count: Count,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GetCartUseCaseError {
    #[error("Cart not found")]
    CartNotFound,
}
