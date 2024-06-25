use common::types::base::generic_types::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    cancel_order::{CancelOrder, CancelOrderUseCaseError},
};

#[derive(new, Debug)]
pub struct CancelOrderUseCase<ShOExtractor, ShOPersister>
where
    ShOExtractor: ShopOrderExtractor,
    ShOPersister: ShopOrderPersister,
{
    shop_order_extractor: AM<ShOExtractor>,
    shop_order_persister: AM<ShOPersister>,
}

impl<ShOExtractor, ShOPersister> CancelOrder for CancelOrderUseCase<ShOExtractor, ShOPersister>
where
    ShOExtractor: ShopOrderExtractor,
    ShOPersister: ShopOrderPersister,
{
    fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), CancelOrderUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_by_id(order_id)
            .map_or(Err(CancelOrderUseCaseError::OrderNotFound), |mut order| {
                order
                    .cancel()
                    .map(|()| self.shop_order_persister.lock().unwrap().save(order))
                    .map_err(|_| CancelOrderUseCaseError::InvalidOrderState)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use domain::test_fixtures::rnd_order_id;

    use super::*;
    use crate::test_fixtures::{
        order_not_ready_for_cancel, order_ready_for_cancel, MockShopOrderExtractor,
        MockShopOrderPersister,
    };

    #[test]
    fn successfully_confirmed() {
        let order = order_ready_for_cancel();

        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));
        extractor.lock().unwrap().order = Some(order.clone());

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(&order.id());

        assert!(result.is_ok());

        let order = &persister.lock().unwrap().order.clone().unwrap();
        persister.lock().unwrap().verify_invoked_order(order);
        persister
            .lock()
            .unwrap()
            .verify_events_after_cancellation(&order.id());
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_cancel();

        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));
        extractor.lock().unwrap().order = Some(order.clone());

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(&order.id());

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order.id());
        assert!(result.is_err());
        assert_eq!(result, Err(CancelOrderUseCaseError::InvalidOrderState));
    }

    #[test]
    fn order_not_found() {
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();

        let result = use_case.execute(&order_id);

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order_id);
        assert!(result.is_err());
        assert_eq!(result, Err(CancelOrderUseCaseError::OrderNotFound));
    }
}
