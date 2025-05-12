use common::types::base::{AM, AMTrait};
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
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
        // Get the order or return NotFound error
        let mut order = self
            .shop_order_extractor
            .lock_un()
            .get_by_id(order_id)
            .ok_or(CancelOrderUseCaseError::OrderNotFound)?;

        // Attempt to cancel the order
        order
            .cancel()
            .map_err(|_| CancelOrderUseCaseError::InvalidOrderState)?;

        // Persist the updated order state
        self.shop_order_persister.lock_un().save(order);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::{AM, AMTrait};
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_cancel,
        order_ready_for_cancel,
    };

    #[test]
    fn successfully_confirmed() {
        let order = order_ready_for_cancel();

        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());
        extractor.lock_un().order = Some(order.clone());

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_ok());

        let order = &persister.lock_un().order.clone().unwrap();
        persister.lock_un().verify_invoked_order(order);
        persister
            .lock_un()
            .verify_events_after_cancellation(order.id());
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_cancel();

        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());
        extractor.lock_un().order = Some(order.clone());

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(order.id());
        assert!(result.is_err());
        assert_eq!(result, Err(CancelOrderUseCaseError::InvalidOrderState));
    }

    #[test]
    fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = CancelOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();

        let result = use_case.execute(&order_id);

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(&order_id);
        assert!(result.is_err());
        assert_eq!(result, Err(CancelOrderUseCaseError::OrderNotFound));
    }
}
