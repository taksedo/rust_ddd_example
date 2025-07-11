use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::{
    cart::value_objects::customer_id::CustomerId,
    order::customer_has_active_order::CustomerHasActiveOrder,
};

use crate::order::access::shop_order_extractor::ShopOrderExtractor;

#[derive(new, Debug)]
pub struct CustomerHasActiveOrderImpl {
    shop_order_extractor: AM<dyn ShopOrderExtractor>,
}

#[async_trait]
impl CustomerHasActiveOrder for CustomerHasActiveOrderImpl {
    async fn invoke(&mut self, for_customer: &CustomerId) -> bool {
        let last_order = self
            .shop_order_extractor
            .lock()
            .await
            .get_last_order(for_customer);
        last_order.is_some() && last_order.unwrap().is_active()
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;
    use tokio::test;

    use super::*;
    use crate::test_fixtures::{MockShopOrderExtractor, active_order, non_active_order};

    #[test]
    async fn active_order_exists() {
        let active_order = active_order();
        let extractor = AM::new_am(MockShopOrderExtractor {
            order: Some(active_order.clone()),
            ..Default::default()
        });
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let has_active_order = rule.invoke(active_order.for_customer()).await;

        assert!(has_active_order);
        extractor
            .lock()
            .await
            .verify_invoked_get_last_order(active_order.for_customer());
    }

    #[test]
    async fn order_exists_but_not_active() {
        let active_order = non_active_order();
        let extractor = AM::new_am(MockShopOrderExtractor {
            order: Some(active_order.clone()),
            ..Default::default()
        });
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let has_active_order = rule.invoke(active_order.for_customer()).await;

        assert!(!has_active_order);
        extractor
            .lock()
            .await
            .verify_invoked_get_last_order(active_order.for_customer());
    }

    #[test]
    async fn order_doesnt_exist() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let mut rule = CustomerHasActiveOrderImpl::new(extractor.clone());

        let customer_id = rnd_customer_id();
        let has_active_order = rule.invoke(&customer_id).await;

        assert!(!has_active_order);
        extractor
            .lock()
            .await
            .verify_invoked_get_last_order(&customer_id);
    }
}
