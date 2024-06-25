use common::types::base::generic_types::AM;
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

impl<CExtractor, CIdGenerator, MExtractor, CPersister> AddMealToCart
    for AddMealToCartUseCase<CExtractor, CIdGenerator, MExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CIdGenerator: CartIdGenerator + 'static,
    MExtractor: MealExtractor,
    CPersister: CartPersister,
{
    fn execute(
        &mut self,
        for_customer: CustomerId,
        meal_id: &MealId,
    ) -> Result<(), AddMealToCartUseCaseError> {
        self.meal_extractor
            .lock()
            .unwrap()
            .get_by_id(meal_id)
            .map_or(Err(AddMealToCartUseCaseError::MealNotFound), |meal| {
                let mut result = self.get_or_create_cart(for_customer);
                result.add_meal(meal);
                Ok(result)
            })
            .map(|cart| self.cart_persister.lock().unwrap().save(cart))
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
    fn get_or_create_cart(&self, for_customer: CustomerId) -> Cart {
        if let Some(result) = self.cart_extractor.lock().unwrap().get_cart(&for_customer) {
            result
        } else {
            Cart::create(self.id_generator.clone(), for_customer)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use domain::{
        cart::value_objects::cart_id::CartId,
        test_fixtures::{rnd_cart_with_customer_id, rnd_customer_id, rnd_meal},
    };

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockCartPersister, MockMealExtractor};

    #[test]
    fn cart_doesnt_exist_successfully_added() {
        let meal = rnd_meal();
        let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        meal_extractor.lock().unwrap().meal = Some(meal.clone());
        let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let customer_id = rnd_customer_id();
        let result = use_case.execute(customer_id, meal.id());

        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());
        cart_persister.lock().unwrap().verify_invoked(
            None,
            Some(&id_generator.lock().unwrap().id),
            Some(meal.id()),
            Some(&customer_id),
        );
        assert!(result.is_ok())
    }

    #[test]
    fn cart_exists_successfully_added() {
        let meal = rnd_meal();
        let customer_id = rnd_customer_id();
        let existing_cart = rnd_cart_with_customer_id(customer_id);

        let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
        meal_extractor.lock().unwrap().meal = Some(meal.clone());
        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(existing_cart.to_owned());

        let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let result = use_case.execute(customer_id, meal.clone().id());
        assert!(result.is_ok());

        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());

        let existing_cart = cart_persister.lock().unwrap().cart.clone().unwrap();

        cart_extractor.lock().unwrap().cart = Some(existing_cart.clone());

        cart_persister.lock().unwrap().verify_invoked(
            Some(&existing_cart),
            None,
            Some(&meal.id()),
            None,
        );
        cart_extractor.lock().unwrap().verify_invoked(&customer_id);
    }

    #[test]
    fn mel_not_found() {
        let meal = rnd_meal();
        let cart_persister = Arc::new(Mutex::new(MockCartPersister::default()));
        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
        let id_generator = Arc::new(Mutex::new(TestCartIdGenerator::default()));

        let mut use_case = AddMealToCartUseCase::new(
            cart_extractor.clone(),
            id_generator.clone(),
            meal_extractor.clone(),
            cart_persister.clone(),
        );

        let result = use_case.execute(rnd_customer_id(), meal.id());

        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());
        cart_persister.lock().unwrap().verify_empty();
        cart_extractor.lock().unwrap().verify_empty();
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
