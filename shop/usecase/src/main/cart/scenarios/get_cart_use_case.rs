use common::types::base::generic_types::AM;
use derive_new::new;
use domain::main::cart::value_objects::customer_id::CustomerId;

use crate::main::{
    cart::{
        access::cart_extractor::CartExtractor,
        get_cart::{CartInfo, CartItem, GetCart, GetCartUseCaseError},
    },
    menu::access::meal_extractor::MealExtractor,
};

#[derive(new, Debug)]
pub struct GetCartUseCase<MExtractor, CExtractor>
where
    MExtractor: MealExtractor,
    CExtractor: CartExtractor,
{
    meal_extractor: AM<MExtractor>,
    cart_extractor: AM<CExtractor>,
}

impl<MExtractor, CExtractor> GetCart for GetCartUseCase<MExtractor, CExtractor>
where
    MExtractor: MealExtractor,
    CExtractor: CartExtractor,
{
    fn execute(&self, for_customer: CustomerId) -> Result<CartInfo, GetCartUseCaseError> {
        let cart = &self.cart_extractor.lock().unwrap().get_cart(&for_customer);
        if let Some(option_value) = cart {
            let cart_item_list = option_value
                .get_meals()
                .iter()
                .map(|(meal_id, count)| {
                    let meal = &self.meal_extractor.lock().unwrap().get_by_id(meal_id);
                    if meal.is_none() {
                        panic!("Meal #{} not found", meal_id.to_i64())
                    }
                    CartItem {
                        meal_id: *meal_id,
                        count: *count,
                        meal_name: meal.clone().unwrap().get_name().to_owned(),
                    }
                })
                .collect();
            Ok(CartInfo::new(for_customer, cart_item_list))
        } else {
            Err(GetCartUseCaseError::CartNotFound)
        }
    }
}
