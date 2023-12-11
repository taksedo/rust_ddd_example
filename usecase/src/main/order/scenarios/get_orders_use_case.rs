use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;
use crate::main::order::dto::order_details::{OrderDetails, ToDetails};
use crate::main::order::get_orders::{GetOrders, GetOrdersUseCaseError};

#[derive(new, Debug)]
pub struct GetOrdersUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    limit: fn() -> i32,
}

impl GetOrders for GetOrdersUseCase {
    fn execute(
        &self,
        start_id: ShopOrderId,
        limit: i32,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError> {
        let curr_limit = (self.limit)();
        if curr_limit < limit {
            Err(GetOrdersUseCaseError::LimitExceed(curr_limit))
        } else {
            Ok(self
                .shop_order_extractor
                .lock()
                .unwrap()
                .get_all(start_id, curr_limit)
                .iter()
                .map(|order| order.to_details())
                .collect())
        }
    }
}
