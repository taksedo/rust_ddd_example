use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    dto::order_details::{AsDetails, OrderDetails},
    get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
};

#[derive(new, Debug)]
pub struct GetOrderByIdUseCase<ShOExtractor: ShopOrderExtractor> {
    shop_order_extractor: AM<ShOExtractor>,
}

#[async_trait]
impl<ShOExtractor: ShopOrderExtractor> GetOrderById for GetOrderByIdUseCase<ShOExtractor> {
    async fn execute(
        &mut self,
        id: &ShopOrderId,
    ) -> Result<OrderDetails, GetOrderByIdUseCaseError> {
        self.shop_order_extractor
            .lock()
            .await
            .get_by_id(id)
            .ok_or(GetOrderByIdUseCaseError::OrderNotFound)
            .map(|order| order.as_details())
    }
}
