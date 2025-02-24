use common::types::base::{AM, ArcMutexTrait};
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    confirm_order::{ConfirmOrder, ConfirmOrderUseCaseError},
};

#[derive(new, Debug)]
pub struct ConfirmOrderUseCase<ShOExtractor, ShOPersister>
where
    ShOExtractor: ShopOrderExtractor,
    ShOPersister: ShopOrderPersister,
{
    shop_order_extractor: AM<ShOExtractor>,
    shop_order_persister: AM<ShOPersister>,
}

impl<ShOExtractor, ShOPersister> ConfirmOrder for ConfirmOrderUseCase<ShOExtractor, ShOPersister>
where
    ShOExtractor: ShopOrderExtractor,
    ShOPersister: ShopOrderPersister,
{
    fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), ConfirmOrderUseCaseError> {
        self.shop_order_extractor
            .lock_un()
            .get_by_id(order_id)
            .map_or(Err(ConfirmOrderUseCaseError::OrderNotFound), |mut order| {
                order
                    .confirm()
                    .map(|_| self.shop_order_persister.lock_un().save(order))
                    .map_err(|_| ConfirmOrderUseCaseError::InvalidOrderState)
            })
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::{AM, ArcMutexTrait};
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_confirm,
        order_ready_for_confirm,
    };

    #[test]
    fn successfully_confirmed() {
        let order = order_ready_for_confirm();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_ok());

        let order = persister.lock_un().order.clone().unwrap();

        persister.lock_un().verify_invoked_order(&order);
        persister
            .lock_un()
            .verify_events_after_confirmation(order.id());
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_confirm();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfirmOrderUseCaseError::InvalidOrderState
        );

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();
        let result = use_case.execute(&order_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfirmOrderUseCaseError::OrderNotFound);

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(&order_id);
    }
}
