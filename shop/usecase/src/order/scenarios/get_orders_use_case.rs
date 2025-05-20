use std::fmt::Debug;

use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    dto::order_details::{AsDetails, OrderDetails},
    get_orders::{GetOrders, GetOrdersUseCaseError},
};

#[derive(new, Debug)]
pub struct GetOrdersUseCase<ShOExtractor: ShopOrderExtractor> {
    shop_order_extractor: AM<ShOExtractor>,
    limit: fn() -> usize,
}

#[async_trait]
impl<ShOExtractor: ShopOrderExtractor> GetOrders for GetOrdersUseCase<ShOExtractor> {
    async fn execute(
        &mut self,
        start_id: &ShopOrderId,
        limit: usize,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError> {
        let max_size = (self.limit)();
        if max_size < limit {
            Err(GetOrdersUseCaseError::LimitExceed(max_size))
        } else {
            Ok(self
                .shop_order_extractor
                .lock()
                .await
                .get_all(start_id, max_size)
                .iter()
                .map(|order| order.as_details())
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;
    use tokio::test;

    use super::*;
    use crate::test_fixtures::MockShopOrderExtractor;

    #[test]
    async fn storage_is_empty() {
        let order_id = rnd_order_id();
        let limit: fn() -> usize = || 10;

        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);

        let result = use_case.execute(&order_id, limit()).await;
        let list = result.unwrap();

        assert!(list.is_empty());
        extractor.lock().await.verify_invoked_get_all();
    }

    #[test]
    async fn storage_is_not_empty() {
        let limit: fn() -> usize = || 10;

        let order = rnd_order(Default::default());
        let order_id = order.id();

        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock().await.order = Some(order.clone());

        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);
        let result = use_case.execute(order_id, limit()).await;
        let list = result.unwrap();

        extractor.lock().await.verify_invoked_get_all();
        assert_eq!(list, vec![order.as_details()]);
    }

    #[test]
    async fn limit_exceed() {
        let limit: fn() -> usize = || 10;
        let order_id = rnd_order_id();

        let extractor = AM::new_am(MockShopOrderExtractor::default());

        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);
        let result = use_case.execute(&order_id, limit() + 1).await;

        assert!(result.is_err());

        assert_eq!(result.unwrap_err(), GetOrdersUseCaseError::LimitExceed(10));
        extractor.lock().await.verify_empty();
    }
}
