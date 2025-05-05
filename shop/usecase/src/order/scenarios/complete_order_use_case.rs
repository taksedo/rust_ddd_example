use common::types::base::{AM, AMTrait};
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    complete_order::{CompleteOrder, CompleteOrderUseCaseError},
};

#[derive(new, Debug)]
pub struct CompleteOrderUseCase {
    shop_order_extractor: AM<dyn ShopOrderExtractor>,
    shop_order_persister: AM<dyn ShopOrderPersister>,
}

impl CompleteOrder for CompleteOrderUseCase {
    fn execute(&self, order_id: &ShopOrderId) -> Result<(), CompleteOrderUseCaseError> {
        self.shop_order_extractor
            .lock_un()
            .get_by_id(order_id)
            .map_or(
                Err(CompleteOrderUseCaseError::OrderNotFound),
                |mut order| {
                    order
                        .complete()
                        .map(|_| self.shop_order_persister.lock_un().save(order))
                        .map_err(|_| CompleteOrderUseCaseError::InvalidOrderState)
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_complete,
        order_ready_for_complete,
    };

    #[test]
    fn successfully_completed() {
        let order = order_ready_for_complete();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_ok());

        let order = persister.lock_un().order.clone().unwrap();
        persister.lock_un().verify_invoked_order(&order);
        persister
            .lock_un()
            .verify_events_after_completion(order.id());
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_complete();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CompleteOrderUseCaseError::InvalidOrderState
        );

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();
        let result = use_case.execute(&order_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CompleteOrderUseCaseError::OrderNotFound
        );

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(&order_id);
    }
}
