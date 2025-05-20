use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::{
    cart::{
        cart::Cart,
        value_objects::{cart_id::CartIdGenerator, customer_id::CustomerId},
    },
    menu::value_objects::meal_id::MealId,
};

use crate::{
    cart::{
        access::{cart_extractor::CartExtractor, cart_persister::CartPersister},
        add_meal_to_cart::{AddMealToCart, AddMealToCartUseCaseError},
    },
    menu::access::meal_extractor::MealExtractor,
};

#[derive(new, Debug)]
pub struct AddMealToCartUseCase<CExtractor, CIdGenerator, MExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CIdGenerator: CartIdGenerator,
    MExtractor: MealExtractor,
    CPersister: CartPersister,
{
    cart_extractor: AM<CExtractor>,
    id_generator: AM<CIdGenerator>,
    meal_extractor: AM<MExtractor>,
    cart_persister: AM<CPersister>,
}

#[async_trait]
impl<CExtractor, CIdGenerator, MExtractor, CPersister> AddMealToCart
    for AddMealToCartUseCase<CExtractor, CIdGenerator, MExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CIdGenerator: CartIdGenerator + 'static,
    MExtractor: MealExtractor,
    CPersister: CartPersister,
{
    async fn execute(
        &mut self,
        for_customer: CustomerId,
        meal_id: &MealId,
    ) -> Result<(), AddMealToCartUseCaseError> {
        // Find meal or return error
        let meal = self
            .meal_extractor
            .lock()
            .await
            .get_by_id(meal_id)
            .ok_or(AddMealToCartUseCaseError::MealNotFound)?;

        // Get or create cart
        let mut cart = self.get_or_create_cart(for_customer).await;

        // Add meal to cart
        cart.add_meal(meal);

        // Persist updated cart
        self.cart_persister.lock().await.save(cart).await;

        Ok(())
    }
}

impl<CExtractor, CIdGenerator, MExtractor, CPersister>
    AddMealToCartUseCase<CExtractor, CIdGenerator, MExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CIdGenerator: CartIdGenerator + 'static,
    MExtractor: MealExtractor,
    CPersister: CartPersister,
{
    async fn get_or_create_cart(&self, for_customer: CustomerId) -> Cart {
        match self
            .cart_extractor
            .lock()
            .await
            .get_cart(&for_customer)
            .await
        {
            Some(result) => result,
            _ => Cart::create(self.id_generator.clone(), for_customer).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use common::types::base::AMTrait;
    use domain::{cart::value_objects::cart_id::CartId, test_fixtures::*};

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockCartPersister, MockMealExtractor};

    #[tokio::test]
    async fn cart_doesnt_exist_successfully_added() {
        let meal = rnd_meal();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::default());
        let meal_extractor = AM::new_am(MockMealExtractor::new());
        meal_extractor.lock().await.meal = Some(meal.clone());
        let id_generator = AM::new_am(TestCartIdGenerator::default());

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let customer_id = rnd_customer_id();
        let result = use_case.execute(customer_id, meal.id()).await;

        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(meal.id());
        cart_persister.lock().await.verify_invoked(
            None,
            Some(&id_generator.lock().await.id),
            Some(meal.id()),
            Some(&customer_id),
        );
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn cart_exists_successfully_added() {
        let meal = rnd_meal();
        let customer_id = rnd_customer_id();
        let existing_cart = rnd_cart_with_customer_id(customer_id);

        let cart_persister = AM::new_am(MockCartPersister::default());
        let meal_extractor = AM::new_am(MockMealExtractor::default());
        meal_extractor.lock().await.meal = Some(meal.clone());
        let cart_extractor = AM::new_am(MockCartExtractor::default());
        cart_extractor.lock().await.cart = Some(existing_cart.to_owned());

        let id_generator = AM::new_am(TestCartIdGenerator::default());

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let result = use_case.execute(customer_id, meal.clone().id()).await;
        assert!(result.is_ok());

        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(meal.id());

        let existing_cart = cart_persister.lock().await.cart.clone().unwrap();

        cart_extractor.lock().await.cart = Some(existing_cart.clone());

        cart_persister.lock().await.verify_invoked(
            Some(&existing_cart),
            None,
            Some(meal.id()),
            None,
        );
        cart_extractor.lock().await.verify_invoked(&customer_id);
    }

    #[tokio::test]
    async fn mel_not_found() {
        let meal = rnd_meal();
        let cart_persister = AM::new_am(MockCartPersister::default());
        let cart_extractor = AM::new_am(MockCartExtractor::default());
        let meal_extractor = AM::new_am(MockMealExtractor::default());
        let id_generator = AM::new_am(TestCartIdGenerator::default());

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let result = use_case.execute(rnd_customer_id(), meal.id()).await;

        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(meal.id());
        cart_persister.lock().await.verify_empty();
        cart_extractor.lock().await.verify_empty();
        assert_eq!(result.unwrap_err(), AddMealToCartUseCaseError::MealNotFound);
    }

    #[derive(new, Debug, Default)]
    struct TestCartIdGenerator {
        id: CartId,
    }

    impl CartIdGenerator for TestCartIdGenerator {
        fn generate(&mut self) -> CartId {
            self.id
        }
    }
}
