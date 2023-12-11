use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;
use crate::main::order::access::shop_order_persister::ShopOrderPersister;
use crate::main::order::pay_order::{PayOrder, PayOrderHandlerError};

#[derive(new, Debug)]
pub struct PayOrderHandler {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl PayOrder for PayOrderHandler {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), PayOrderHandlerError> {
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
