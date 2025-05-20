use async_trait::async_trait;
use common::types::base::AM;
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

#[async_trait]
impl<ShOExtractor, ShOPersister> ConfirmOrder for ConfirmOrderUseCase<ShOExtractor, ShOPersister>
where
    ShOExtractor: ShopOrderExtractor,
    ShOPersister: ShopOrderPersister,
{
    async fn execute(&mut self, order_id: &ShopOrderId) -> Result<(), ConfirmOrderUseCaseError> {
        // Get order or return NotFound error
        let mut order = self
            .shop_order_extractor
            .lock()
            .await
            .get_by_id(order_id)
            .ok_or(ConfirmOrderUseCaseError::OrderNotFound)?;

        // Confirm order state
        order
            .confirm()
            .map_err(|_| ConfirmOrderUseCaseError::InvalidOrderState)?;

        // Persist updated order
        self.shop_order_persister.lock().await.save(order).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;
    use tokio::test;

    use super::*;
    use crate::test_fixtures::{
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_confirm,
        order_ready_for_confirm,
    };

    #[test]
    async fn successfully_confirmed() {
        let order = order_ready_for_confirm();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock().await.order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id()).await;

        assert!(result.is_ok());

        let order = persister.lock().await.order.clone().unwrap();

        persister.lock().await.verify_invoked_order(&order);
        persister
            .lock()
            .await
            .verify_events_after_confirmation(order.id());
        extractor.lock().await.verify_invoked_get_by_id(order.id());
    }

    #[test]
    async fn invalid_state() {
        let order = order_not_ready_for_confirm();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock().await.order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());
        let result = use_case.execute(order.id()).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfirmOrderUseCaseError::InvalidOrderState
        );

        persister.lock().await.verify_empty();
        extractor.lock().await.verify_invoked_get_by_id(order.id());
    }

    #[test]
    async fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let mut use_case = ConfirmOrderUseCase::new(extractor.clone(), persister.clone());

        let order_id = rnd_order_id();
        let result = use_case.execute(&order_id).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ConfirmOrderUseCaseError::OrderNotFound);

        persister.lock().await.verify_empty();
        extractor.lock().await.verify_invoked_get_by_id(&order_id);
    }
}
