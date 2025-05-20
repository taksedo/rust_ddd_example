use async_trait::async_trait;
use common::types::base::AM;
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

#[async_trait]
impl<CExtractor, CPersister> RemoveMealFromCart
    for RemoveMealFromCartUseCase<CExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CPersister: CartPersister,
{
    async fn execute(
        &self,
        for_customer: &CustomerId,
        meal_id: &MealId,
    ) -> Result<(), RemoveMealFromCartUseCaseError> {
        // Get the cart or return error
        let mut cart = self
            .cart_extractor
            .lock()
            .await
            .get_cart(for_customer)
            .await
            .ok_or(RemoveMealFromCartUseCaseError::CartNotFound)?;

        // Remove meals and persist changes
        cart.remove_meals(meal_id);
        self.cart_persister.lock().await.save(cart).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockCartPersister};
    #[tokio::test]
    async fn successfully_removed() {
        let cart = rnd_cart();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::new(Some(cart.clone()), None));

        let use_case =
            RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
        let result = use_case
            .execute(cart.clone().for_customer(), &rnd_meal_id())
            .await;

        cart_extractor
            .lock()
            .await
            .verify_invoked(cart.for_customer());
        cart_persister
            .lock()
            .await
            .verify_invoked(Some(&cart), None, None, None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn cart_not_found() {
        let cart = rnd_cart();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::default());

        let use_case =
            RemoveMealFromCartUseCase::new(cart_extractor.clone(), cart_persister.clone());
        let result = use_case
            .execute(cart.clone().for_customer(), &rnd_meal_id())
            .await;

        cart_extractor
            .lock()
            .await
            .verify_invoked(cart.clone().for_customer());
        cart_persister.lock().await.verify_empty();
        assert_eq!(
            result.unwrap_err(),
            RemoveMealFromCartUseCaseError::CartNotFound
        );
    }
}
