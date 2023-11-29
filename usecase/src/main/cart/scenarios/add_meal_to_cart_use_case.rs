use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::cart::cart::Cart;
use domain::main::cart::value_objects::cart_id::CartIdGenerator;
use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::meal_id::MealId;

use crate::main::cart::access::cart_extractor::CartExtractor;
use crate::main::cart::access::cart_persister::CartPersister;
use crate::main::cart::add_meal_to_cart::{AddMealToCart, AddMealToCartUseCaseError};
use crate::main::menu::access::meal_extractor::MealExtractor;

#[derive(new, Debug)]
pub struct AddMealToCartUseCase {
    cart_extractor: Arc<Mutex<dyn CartExtractor>>,
    id_generator: Arc<Mutex<dyn CartIdGenerator>>,
    meal_extractor: Arc<Mutex<dyn MealExtractor>>,
    cart_persister: Arc<Mutex<dyn CartPersister>>,
}

impl AddMealToCart for AddMealToCartUseCase {
    fn execute(
        &mut self,
        for_customer: CustomerId,
        meal_id: MealId,
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

impl AddMealToCartUseCase {
    fn get_or_create_cart(&self, for_customer: CustomerId) -> Cart {
        if let Some(result) = self
            .cart_extractor
            .lock()
            .unwrap()
            .get_cart(for_customer.clone())
        {
            result
        } else {
            Cart::create(Arc::clone(&self.id_generator), for_customer)
        }
    }
}
