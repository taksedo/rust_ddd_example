use common::types::base::generic_types::AM;
use derive_new::new;
use domain::main::{
    cart::value_objects::customer_id::CustomerId, menu::value_objects::meal_id::MealId,
};

use crate::main::cart::{
    access::{cart_extractor::CartExtractor, cart_persister::CartPersister},
    remove_meal_from_cart::{RemoveMealFromCart, RemoveMealFromCartUseCaseError},
};

#[derive(new, Debug)]
pub struct RemoveMealFromCartUseCase<CExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CPersister: CartPersister,
{
    cart_extractor: AM<CExtractor>,
    cart_persister: AM<CPersister>,
}

impl<CExtractor, CPersister> RemoveMealFromCart
    for RemoveMealFromCartUseCase<CExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CPersister: CartPersister,
{
    fn execute(
        &self,
        for_customer: CustomerId,
        meal_id: MealId,
    ) -> Result<(), RemoveMealFromCartUseCaseError> {
        self.cart_extractor
            .lock()
            .unwrap()
            .get_cart(for_customer)
            .map_or(
                Err(RemoveMealFromCartUseCaseError::CartNotFound),
                |mut cart| {
                    {
                        cart.remove_meals(meal_id);
                        self.cart_persister.lock().unwrap().save(cart)
                    };
                    Ok(())
                },
            )
    }
}
