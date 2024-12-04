use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    complete_order::{CompleteOrder, CompleteOrderUseCaseError},
};

#[derive(new, Debug)]
pub struct CompleteOrderUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl CompleteOrder for CompleteOrderUseCase {
    fn execute(&self, order_id: &ShopOrderId) -> Result<(), CompleteOrderUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_by_id(order_id)
            .map_or(
                Err(CompleteOrderUseCaseError::OrderNotFound),
                |mut order| {
                    order
                        .complete()
                        .map(|_| self.shop_order_persister.lock().unwrap().save(order))
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
        order_not_ready_for_complete, order_ready_for_complete, MockShopOrderExtractor,
        MockShopOrderPersister,
    };

    #[test]
    fn successfully_completed() {
        let order = order_ready_for_complete();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        extractor.lock().unwrap().order = Some(order.clone());
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_ok());

        let order = persister.lock().unwrap().order.clone().unwrap();
        persister.lock().unwrap().verify_invoked_order(&order);
        persister
            .lock()
            .unwrap()
            .verify_events_after_completion(order.id());
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_complete();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        extractor.lock().unwrap().order = Some(order.clone());
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CompleteOrderUseCaseError::InvalidOrderState
        );

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn order_not_found() {
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let use_case = CompleteOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();
        let result = use_case.execute(&order_id);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CompleteOrderUseCaseError::OrderNotFound
        );

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order_id);
    }
}
