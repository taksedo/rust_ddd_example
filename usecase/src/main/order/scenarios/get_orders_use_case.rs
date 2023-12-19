use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use derive_new::new;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    dto::order_details::{OrderDetails, ToDetails},
    get_orders::{GetOrders, GetOrdersUseCaseError},
};

#[derive(new, Debug)]
pub struct GetOrdersUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    limit: fn() -> usize,
}

impl GetOrders for GetOrdersUseCase {
    fn execute(
        &mut self,
        start_id: ShopOrderId,
        limit: usize,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError> {
        let max_size = (self.limit)();
        if max_size < limit {
            Err(GetOrdersUseCaseError::LimitExceed(max_size))
        } else {
            Ok(self
                .shop_order_extractor
                .lock()
                .unwrap()
                .get_all(start_id, max_size)
                .iter()
                .map(|order| order.to_details())
                .collect())
        }
    }
}
