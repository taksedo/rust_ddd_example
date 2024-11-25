use std::mem::{discriminant, Discriminant};

use common::{events::domain_event_listener::DomainEventListener, types::base::generic_types::AM};
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
            .lock()
            .unwrap()
            .get_cart(&event_struct.for_customer);

        if result.is_none() {
            let _ = tracing_subscriber::fmt::try_init();
            info!(
                "Cart for customer #{} is already removed",
                event_struct.for_customer
            )
        } else {
            self.cart_remover
                .lock()
                .unwrap()
                .delete_cart(result.clone().unwrap())
        }
    }

    fn get_events(&self) -> &Vec<ShopOrderEventEnum> {
        todo!()
    }
}

#[cfg(all(test, feature = "usecase"))]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing_test::traced_test;

    use super::*;
    use crate::{
        domain_test_fixtures::{rnd_cart, rnd_customer_id, rnd_order_id, rnd_price},
        test_fixtures::{MockCartExtractor, MockCartRemover},
    };

    #[test]
    fn successfully_removed() {
        let cart_remover = Arc::new(Mutex::new(MockCartRemover::default()));
        let cart = rnd_cart();

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(cart.clone());

        let mut rule =
            RemoveCartAfterCheckoutRule::new(cart_extractor.clone(), cart_remover.clone());
        let event: ShopOrderEventEnum = ShopOrderCreatedDomainEvent::new(
            rnd_order_id(),
            *cart.clone().for_customer(),
            rnd_price(),
        )
        .into();

        rule.handle(&event);

        cart_extractor
            .lock()
            .unwrap()
            .verify_invoked(cart.for_customer());
        cart_remover.lock().unwrap().verify_invoked(cart.id());
    }

    #[test]
    #[traced_test]
    fn cart_not_found() {
        let cart_remover = Arc::new(Mutex::new(MockCartRemover::default()));

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));

        let mut rule =
            RemoveCartAfterCheckoutRule::new(cart_extractor.clone(), cart_remover.clone());
        let customer_id = rnd_customer_id();
        let event: ShopOrderEventEnum =
            ShopOrderCreatedDomainEvent::new(rnd_order_id(), customer_id, rnd_price()).into();

        rule.handle(&event);

        cart_extractor.lock().unwrap().verify_invoked(&customer_id);
        cart_remover.lock().unwrap().verify_empty();

        assert!(logs_contain(&format!(
            "Cart for customer #{customer_id} is already removed"
        )));
    }
}
