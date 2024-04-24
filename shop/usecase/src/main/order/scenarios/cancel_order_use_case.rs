use common::types::base::generic_types::AM;
use derive_new::new;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

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
    fn execute(&mut self, order_id: ShopOrderId) -> Result<(), CancelOrderUseCaseError> {
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
