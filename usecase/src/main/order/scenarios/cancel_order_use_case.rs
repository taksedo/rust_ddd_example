use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;
use crate::main::order::access::shop_order_persister::ShopOrderPersister;
use crate::main::order::cancel_order::{CancelOrder, CancelOrderUseCaseError};

#[derive(new, Debug)]
pub struct CancelOrderUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl CancelOrder for CancelOrderUseCase {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), CancelOrderUseCaseError> {
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
