use std::fmt::Debug;

use async_trait::async_trait;

use crate::cart::value_objects::customer_id::CustomerId;

#[async_trait]
pub trait CustomerHasActiveOrder: Debug + Send {
    async fn invoke(&mut self, for_customer: &CustomerId) -> bool;
}
