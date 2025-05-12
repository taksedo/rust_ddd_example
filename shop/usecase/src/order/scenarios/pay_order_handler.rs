use common::types::base::{AM, AMTrait};
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

impl PayOrder for PayOrderHandler {
    fn execute(&self, order_id: &ShopOrderId) -> Result<(), PayOrderHandlerError> {
        let mut order = self
            .shop_order_extractor
            .lock_un()
            .get_by_id(order_id)
            .ok_or(PayOrderHandlerError::OrderNotFound)?;

        order
            .pay()
            .map_err(|_| PayOrderHandlerError::InvalidOrderState)?;

        self.shop_order_persister.lock_un().save(order);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{
        MockShopOrderExtractor, MockShopOrderPersister, order_not_ready_for_pay,
        order_ready_for_pay,
    };

    #[test]
    fn successfully_payed() {
        let order = order_ready_for_pay();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(order.id());

        assert!(result.is_ok());

        let order = persister.lock_un().order.clone().unwrap();

        persister.lock_un().verify_invoked_order(&order);
        extractor.lock_un().verify_invoked_get_by_id(order.id());
        persister.lock_un().verify_events_after_payment(order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_pay();
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(order.id());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::InvalidOrderState);

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }

    #[test]
    fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let persister = AM::new_am(MockShopOrderPersister::default());

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let order_id = rnd_order_id();
        let result = handler.execute(&order_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::OrderNotFound);

        persister.lock_un().verify_empty();
        extractor.lock_un().verify_invoked_get_by_id(&order_id);
    }
}
