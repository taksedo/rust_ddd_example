use std::sync::{Arc, Mutex};

use derive_new::new;
use domain::{
    cart::value_objects::customer_id::CustomerId,
    order::customer_has_active_order::CustomerHasActiveOrder,
};

use crate::order::access::shop_order_extractor::ShopOrderExtractor;

#[derive(new, Debug)]
pub struct CustomerHasActiveOrderImpl {
    shop_order_extractor: Arc<Mutex<dyn ShopOrderExtractor>>,
}

impl CustomerHasActiveOrder for CustomerHasActiveOrderImpl {
    fn invoke(&mut self, for_customer: &CustomerId) -> bool {
        let last_order = self
            .shop_order_extractor
            .lock()
            .unwrap()
            .get_last_order(for_customer);
        last_order.is_some() && last_order.unwrap().is_active()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use domain::order::customer_has_active_order::CustomerHasActiveOrder;
    use domain_test_fixtures::rnd_customer_id;
    use usecase::order::invariants::customer_has_active_order_impl::CustomerHasActiveOrderImpl;
    use usecase_test_fixtures::{active_order, non_active_order, MockShopOrderExtractor};

    #[test]
    fn active_order_exists() {
        let active_order = active_order();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor {
            order: Some(active_order.clone()),
            ..Default::default()
        }));
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let has_active_order = rule.invoke(active_order.for_customer());

        assert!(has_active_order);
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_last_order(active_order.for_customer());
    }

    #[test]
    fn order_exists_but_not_active() {
        let active_order = non_active_order();
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor {
            order: Some(active_order.clone()),
            ..Default::default()
        }));
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let has_active_order = rule.invoke(active_order.for_customer());

        assert!(!has_active_order);
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_last_order(active_order.for_customer());
    }

    #[test]
    fn order_doesnt_exist() {
        let extractor = Arc::new(Mutex::new(MockShopOrderExtractor::default()));
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let customer_id = rnd_customer_id();
        let has_active_order = rule.invoke(&customer_id);

        assert!(!has_active_order);
        extractor
            .lock()
            .unwrap()
            .verify_invoked_get_last_order(&customer_id);
    }
}
