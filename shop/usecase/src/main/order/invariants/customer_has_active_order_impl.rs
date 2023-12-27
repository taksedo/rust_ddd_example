use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::main::{
    cart::value_objects::customer_id::CustomerId,
    order::customer_has_active_order::CustomerHasActiveOrder,
};

use crate::main::order::access::shop_order_extractor::ShopOrderExtractor;

#[derive(new, Debug)]
pub struct CustomerHasActiveOrderImpl {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
}

impl CustomerHasActiveOrder for CustomerHasActiveOrderImpl {
    fn invoke(&mut self, for_customer: CustomerId) -> bool {
        let last_order = self
            .shop_order_extractor
            .lock()
            .unwrap()
            .get_last_order(for_customer);
        last_order.is_some() && last_order.unwrap().is_active()
    }
}
