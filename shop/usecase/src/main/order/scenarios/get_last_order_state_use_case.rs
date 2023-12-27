use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::main::{cart::value_objects::customer_id::CustomerId, order::shop_order::OrderState};

use crate::main::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    get_last_order_state::{GetLastOrderState, GetLastOrderStateUseCaseError},
};

#[derive(new, Debug)]
pub struct GetLastOrderStateUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
}

impl GetLastOrderState for GetLastOrderStateUseCase {
    fn execute(
        &self,
        for_customer: CustomerId,
    ) -> Result<OrderState, GetLastOrderStateUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_last_order(for_customer)
            .map_or(Err(GetLastOrderStateUseCaseError::OrderNotFound), |order| {
                Ok(order.state)
            })
    }
}