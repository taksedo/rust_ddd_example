use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::{
    access::{shop_order_extractor::ShopOrderExtractor, shop_order_persister::ShopOrderPersister},
    complete_order::{CompleteOrder, CompleteOrderUseCaseError},
};

#[derive(new, Debug)]
pub struct CompleteOrderUseCase {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl CompleteOrder for CompleteOrderUseCase {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), CompleteOrderUseCaseError> {
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