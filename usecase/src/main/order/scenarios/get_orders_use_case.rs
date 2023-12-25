use std::fmt::Debug;

use common::types::main::base::generic_types::AM;
use derive_new::new;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    dto::order_details::{OrderDetails, ToDetails},
    get_orders::{GetOrders, GetOrdersUseCaseError},
};

#[derive(new, Debug)]
pub struct GetOrdersUseCase<ShOExtractor: ShopOrderExtractor> {
    shop_order_extractor: AM<ShOExtractor>,
    limit: fn() -> usize,
}

impl<ShOExtractor: ShopOrderExtractor> GetOrders for GetOrdersUseCase<ShOExtractor> {
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
