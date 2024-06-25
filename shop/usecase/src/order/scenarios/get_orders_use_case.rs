use std::fmt::Debug;

use common::types::base::generic_types::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
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
                .unwrap()
                .get_all(start_id, max_size)
                .iter()
                .map(|order| order.to_details())
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use domain::test_fixtures::{rnd_order, rnd_order_id};

    use super::*;
    use crate::test_fixtures::MockShopOrderExtractor;

    #[test]
    fn storage_is_empty() {
        let order_id = rnd_order_id();
        let limit: fn() -> usize = || 10;

        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);

        let result = use_case.execute(&order_id, limit());
        let list = result.unwrap();

        assert!(list.is_empty());
        extractor.lock().unwrap().verify_invoked_get_all();
    }

    #[test]
    fn storage_is_not_empty() {
        let limit: fn() -> usize = || 10;

        let order = rnd_order(Default::default());
        let order_id = order.id();

        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        extractor.lock().unwrap().order = Some(order.clone());

        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);
        let result = use_case.execute(&order_id, limit());
        let list = result.unwrap();

        extractor.lock().unwrap().verify_invoked_get_all();
        assert_eq!(list, vec![order.to_details()]);
    }

    #[test]
    fn limit_exceed() {
        let limit: fn() -> usize = || 10;
        let order_id = rnd_order_id();

        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));

        let mut use_case = GetOrdersUseCase::new(extractor.clone(), limit);
        let result = use_case.execute(&order_id, limit() + 1);

        assert!(result.is_err());

        assert_eq!(result.unwrap_err(), GetOrdersUseCaseError::LimitExceed(10));
        extractor.lock().unwrap().verify_empty();
    }
}
