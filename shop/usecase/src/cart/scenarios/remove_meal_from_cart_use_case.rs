use common::types::base::{AM, AMTrait};
use derive_new::new;
use domain::{cart::value_objects::customer_id::CustomerId, menu::value_objects::meal_id::MealId};

use crate::cart::{
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
        for_customer: &CustomerId,
        meal_id: &MealId,
    ) -> Result<(), RemoveMealFromCartUseCaseError> {
        self.cart_extractor.lock_un().get_cart(for_customer).map_or(
            Err(RemoveMealFromCartUseCaseError::CartNotFound),
            |mut cart| {
                {
                    cart.remove_meals(meal_id);
                    self.cart_persister.lock_un().save(cart)
                };
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockCartPersister};
    #[test]
    fn successfully_removed() {
        let cart = rnd_cart();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::new(Some(cart.clone()), None));

        let use_case =
            RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
        let result = use_case.execute(cart.clone().for_customer(), &rnd_meal_id());

        cart_extractor.lock_un().verify_invoked(cart.for_customer());
        cart_persister
            .lock_un()
            .verify_invoked(Some(&cart), None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn cart_not_found() {
        let cart = rnd_cart();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::default());

        let use_case =
            RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
        let result = use_case.execute(cart.clone().for_customer(), &rnd_meal_id());

        cart_extractor
            .lock_un()
            .verify_invoked(cart.clone().for_customer());
        cart_persister.lock_un().verify_empty();
        assert_eq!(
            result.unwrap_err(),
            RemoveMealFromCartUseCaseError::CartNotFound
        );
    }
}
