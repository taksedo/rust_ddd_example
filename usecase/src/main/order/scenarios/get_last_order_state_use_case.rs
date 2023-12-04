use std::sync::{Arc, Mutex};

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::order::shop_order::OrderState;

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;
use crate::main::order::get_last_order_state::{GetLastOrderState, GetLastOrderStateUseCaseError};

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
