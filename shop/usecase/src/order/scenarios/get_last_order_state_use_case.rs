use common::types::base::AM;
use derive_new::new;
use domain::{cart::value_objects::customer_id::CustomerId, order::shop_order::OrderState};

use crate::order::{
    access::shop_order_extractor::ShopOrderExtractor,
    get_last_order_state::{GetLastOrderState, GetLastOrderStateUseCaseError},
};

#[derive(new, Debug)]
pub struct GetLastOrderStateUseCase {
    shop_order_extractor: AM<dyn ShopOrderExtractor>,
}

impl GetLastOrderState for GetLastOrderStateUseCase {
    fn execute(
        &self,
        for_customer: &CustomerId,
    ) -> Result<OrderState, GetLastOrderStateUseCaseError> {
        self.shop_order_extractor
            .lock()
            .unwrap()
            .get_last_order(for_customer)
            .map_or(Err(GetLastOrderStateUseCaseError::OrderNotFound), |order| {
                Ok(order.state().clone())
            })
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::{AM, AMTrait};
    use domain::test_fixtures::*;

    use super::*;
    use crate::{
        order::{
            get_order_by_id::{GetOrderById, GetOrderByIdUseCaseError},
            scenarios::get_order_by_id_use_case::GetOrderByIdUseCase,
        },
        test_fixtures::MockShopOrderExtractor,
    };

    #[test]
    fn status_successfully_received() {
        let order = rnd_order(Default::default());
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());

        let use_case = GetLastOrderStateUseCase::new(extractor.clone());
        let result = use_case.execute(order.for_customer());

        extractor
            .lock_un()
            .verify_invoked_get_last_order(order.for_customer());
        assert!(result.is_ok());
        assert_eq!(&result.unwrap(), order.state())
    }

    #[test]
    fn order_not_found() {
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        let mut use_case = GetOrderByIdUseCase::new(extractor.clone());

        let order_id = rnd_order_id();
        let result = use_case.execute(&order_id);

        extractor.lock_un().verify_invoked_get_by_id(&order_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetOrderByIdUseCaseError::OrderNotFound)
    }

    #[test]
    fn order_expected_successfully() {
        let order = rnd_order(Default::default());
        let extractor = AM::new_am(MockShopOrderExtractor::default());
        extractor.lock_un().order = Some(order.clone());
        let mut use_case = GetOrderByIdUseCase::new(extractor.clone());

        let result = use_case.execute(order.id());
        assert!(result.is_ok());
        let details = result.unwrap();

        assert_eq!(&details.id, order.id());
        assert_eq!(&details.address, order.address());
        assert_eq!(&details.state, order.state());
        assert_eq!(details.total, order.total_price());
        assert_eq!(
            details.ready_for_confirm_or_cancel,
            order.ready_for_confirm_or_cancel()
        );
        assert_eq!(details.items.len(), order.order_items().len());

        details.items.iter().for_each(|i| {
            let src_item: Vec<_> = order
                .order_items()
                .iter()
                .filter(|&it| it.meal_id == i.meal_id && it.count == i.count)
                .collect();
            assert_eq!(src_item.len(), 1);
        });
        extractor.lock_un().verify_invoked_get_by_id(order.id());
    }
}
