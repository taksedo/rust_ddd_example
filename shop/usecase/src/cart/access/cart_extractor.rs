use std::fmt::Debug;

use async_trait::async_trait;
use domain::cart::{cart::Cart, value_objects::customer_id::CustomerId};

#[async_trait]
pub trait CartExtractor: Debug + Send {
    async fn get_cart(&mut self, for_customer: &CustomerId) -> Option<Cart>;
}
