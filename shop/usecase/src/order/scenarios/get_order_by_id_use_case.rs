use common::types::base::generic_types::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    dto::order_details::{OrderDetails, ToDetails},
    get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
};

#[derive(new, Debug)]
pub struct GetOrderByIdUseCase<ShOExtractor: ShopOrderExtractor> {
    shop_order_extractor: AM<ShOExtractor>,
}

impl<ShOExtractor: ShopOrderExtractor> GetOrderById for GetOrderByIdUseCase<ShOExtractor> {
    fn execute(&mut self, id: &ShopOrderId) -> Result<OrderDetails, GetOrderByIdUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_by_id(id)
            .map_or(Err(GetOrderByIdUseCaseError::OrderNotFound), |order| {
                Ok(order.to_details())
            })
    }
}
