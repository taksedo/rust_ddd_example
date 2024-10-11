use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::order::value_objects::shop_order_id::ShopOrderId;

use crate::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    pay_order::{PayOrder, PayOrderHandlerError},
};

#[derive(new, Debug)]
pub struct PayOrderHandler {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl PayOrder for PayOrderHandler {
    fn execute(&self, order_id: &ShopOrderId) -> Result<(), PayOrderHandlerError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_by_id(order_id)
            .map_or(Err(PayOrderHandlerError::OrderNotFound), |mut order| {
                order
                    .pay()
                    .map(|_| self.shop_order_persister.lock().unwrap().save(order))
                    .map_err(|_| PayOrderHandlerError::InvalidOrderState)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use domain_test_fixtures::rnd_order_id;
    use usecase::order::{
        pay_order::{PayOrder, PayOrderHandlerError},
        scenarios::pay_order_handler::PayOrderHandler,
    };
    use usecase_test_fixtures::{
        order_not_ready_for_pay, order_ready_for_pay, MockShopOrderExtractor,
        MockShopOrderPersister,
    };
    #[test]
    fn successfully_payed() {
        let order = order_ready_for_pay();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        extractor.lock().unwrap().order = Some(order.clone());
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(&order.id());

        assert!(result.is_ok());

        let order = persister.lock().unwrap().order.clone().unwrap();

        persister.lock().unwrap().verify_invoked_order(&order);
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order.id());
        persister
            .lock()
            .unwrap()
            .verify_events_after_payment(&order.id());
    }

    #[test]
    fn invalid_state() {
        let order = order_not_ready_for_pay();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        extractor.lock().unwrap().order = Some(order.clone());
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let result = handler.execute(&order.id());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::InvalidOrderState);

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order.id());
    }

    #[test]
    fn order_not_found() {
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let handler = PayOrderHandler::new(extractor.clone(), persister.clone());
        let order_id = rnd_order_id();
        let result = handler.execute(&order_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PayOrderHandlerError::OrderNotFound);

        persister.lock().unwrap().verify_empty();
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&order_id);
    }
}
