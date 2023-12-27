use std::mem::{discriminant, Discriminant};

use common::{
    events::main::domain_event_listener::DomainEventListener, types::main::base::generic_types::AM,
};
use derive_new::new;
use domain::main::order::customer_order_events::{ShopOrderCreatedDomainEvent, ShopOrderEventEnum};
use tracing::info;

use crate::main::cart::access::{cart_extractor::CartExtractor, cart_remover::CartRemover};

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
            .get_cart(event_struct.clone().for_customer);

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
