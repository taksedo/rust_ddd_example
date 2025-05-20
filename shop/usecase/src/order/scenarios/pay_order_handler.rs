use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    pay_order::{PayOrder, PayOrderHandlerError},
};

#[derive(new, Debug)]
pub struct PayOrderHandler {
    shop_order_extractor: AM<dyn ShopOrderExtractor>,
    shop_order_persister: AM<dyn ShopOrderPersister>,
}

#[async_trait]
impl PayOrder for PayOrderHandler {
    async fn execute(&self, order_id: &ShopOrderId) -> Result<(), PayOrderHandlerError> {
        let mut order = self
            .shop_order_extractor
            .lock()
            .await
            .get_by_id(order_id)
            .ok_or(PayOrderHandlerError::OrderNotFound)?;

        order
            .pay()
            .map_err(|_| PayOrderHandlerError::InvalidOrderState)?;

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
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_pay,
        order_ready_for_pay,
    };

    #[test]
    async fn successfully_payed() {
        let order = order_ready_for_pay();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock().await.order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(order.id()).await;

        assert!(result.is_ok());

        let order = persister.lock().await.order.clone().unwrap();

        persister.lock().await.verify_invoked_order(&order);
        extractor.lock().await.verify_invoked_get_by_id(order.id());
        persister
            .lock()
            .await
            .verify_events_after_payment(order.id());
    }

    #[test]
    async fn invalid_state() {
        let order = order_not_ready_for_pay();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock().await.order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(order.id()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::InvalidOrderState);

        persister.lock().await.verify_empty();
        extractor.lock().await.verify_invoked_get_by_id(order.id());
    }

    #[test]
    async fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let order_id = rnd_order_id();
        let result = handler.execute(&order_id).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::OrderNotFound);

        persister.lock().await.verify_empty();
        extractor.lock().await.verify_invoked_get_by_id(&order_id);
    }
}
