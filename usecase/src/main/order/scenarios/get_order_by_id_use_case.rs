use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;
use crate::main::order::dto::order_details::{OrderDetails, ToDetails};
use crate::main::order::get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError};

#[derive(new, Debug)]
pub struct GetOrderByIdUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
}

impl GetOrderById for GetOrderByIdUseCase {
    fn execute(&self, id: ShopOrderId) -> Result<OrderDetails, GetOrderByIdUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_by_id(id)
            .map_or(Err(GetOrderByIdUseCaseError::OrderNotFound), |order| {
                Ok(order.to_details())
            })
    }
}
