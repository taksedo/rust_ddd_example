use std::mem::{Discriminant, discriminant};

use common::{
    events::DomainEventListener,
    types::base::{AM, AMTrait},
};
use derive_new::new;
use domain::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};
use tracing::info;

use crate::cart::access::{cart_extractor::CartExtractor, cart_remover::CartRemover};

#[derive(new, Debug)]
pub struct RemoveCartAfterCheckoutRule<CExtractor, CRemover>
where
    CExtractor: CartExtractor,
    CRemover: CartRemover,
{
    cart_extractor: AM<CExtractor>,
    cart_remover: AM<CRemover>,
}

impl<CExtractor, CRemover> DomainEventListener<ShopOrderEventEnum>
    for RemoveCartAfterCheckoutRule<CExtractor, CRemover>
where
    CExtractor: CartExtractor,
    CRemover: CartRemover,
{
    fn event_type(&self) -> Discriminant<ShopOrderEventEnum> {
        let event: ShopOrderEventEnum = ShopOrderCreatedDomainEvent::default().into();
        discriminant(&event)
    }

    fn handle(&mut self, event: &ShopOrderEventEnum) {
        let event_struct: ShopOrderCreatedDomainEvent =
            event.clone().try_into().expect("Wrong type of event");

        let result = &self
            .cart_extractor
            .lock_un()
            .get_cart(&event_struct.for_customer);

        if result.is_none() {
            let _ = tracing_subscriber::fmt::try_init();
            info!(
                "Cart for customer #{} is already removed",
                event_struct.for_customer
            )
        } else {
            self.cart_remover
                .lock_un()
                .delete_cart(result.clone().unwrap())
        }
    }

    fn get_events(&self) -> &Vec<ShopOrderEventEnum> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::{AM, AMTrait};
    use domain::test_fixtures::*;
    use tracing_test::traced_test;

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockCartRemover};

    #[test]
    fn successfully_removed() {
        let cart_remover = AM::new_am(MockCartRemover::default());
        let cart = rnd_cart();

        let cart_extractor = AM::new_am(MockCartExtractor::default());
        cart_extractor.lock_un().cart = Some(cart.clone());

        let mut rule =
            RemoveCartAfterCheckoutRule::new(cart_extractor.clone(), cart_remover.clone());
        let event: ShopOrderEventEnum = ShopOrderCreatedDomainEvent::new(
            rnd_order_id(),
            *cart.clone().for_customer(),
            rnd_price(),
        )
        .into();

        rule.handle(&event);

        cart_extractor.lock_un().verify_invoked(cart.for_customer());
        cart_remover.lock_un().verify_invoked(cart.id());
    }

    #[test]
    #[traced_test]
    fn cart_not_found() {
        let cart_remover = AM::new_am(MockCartRemover::default());

        let cart_extractor = AM::new_am(MockCartExtractor::default());

        let mut rule =
            RemoveCartAfterCheckoutRule::new(cart_extractor.clone(), cart_remover.clone());
        let customer_id = rnd_customer_id();
        let event: ShopOrderEventEnum =
            ShopOrderCreatedDomainEvent::new(rnd_order_id(), customer_id, rnd_price()).into();

        rule.handle(&event);

        cart_extractor.lock_un().verify_invoked(&customer_id);
        cart_remover.lock_un().verify_empty();

        assert!(logs_contain(&format!(
            "Cart for customer #{customer_id} is already removed"
        )));
    }
}
