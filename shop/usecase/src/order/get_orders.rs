use std::fmt::Debug;

use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;
use thiserror::Error;

use crate::order::dto::order_details::OrderDetails;

pub trait GetOrders: Debug + Send {
    fn execute(
        &mut self,
        start_id: &ShopOrderId,
        limit: usize,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError>;
}

#[derive(new, Error, Debug, Clone, Copy, PartialEq)]
pub enum GetOrdersUseCaseError {
    #[error("Limit is exceeded")]
    LimitExceed(usize),
}
