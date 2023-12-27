use common::types::main::base::generic_types::AM;
use derive_new::new;
use domain::main::{
    cart::{
        cart::Cart,
        value_objects::{cart_id::CartIdGenerator, customer_id::CustomerId},
    },
    menu::value_objects::meal_id::MealId,
};

use crate::main::{
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

impl<CExtractor, CIdGenerator, MExtractor, CPersister>
    AddMealToCartUseCase<CExtractor, CIdGenerator, MExtractor, CPersister>
where
    CExtractor: CartExtractor,
    CIdGenerator: CartIdGenerator + 'static,
    MExtractor: MealExtractor,
    CPersister: CartPersister,
{
    fn get_or_create_cart(&self, for_customer: CustomerId) -> Cart {
        if let Some(result) = self.cart_extractor.lock().unwrap().get_cart(for_customer) {
            result
        } else {
            Cart::create(self.id_generator.clone(), for_customer)
        }
    }
}
