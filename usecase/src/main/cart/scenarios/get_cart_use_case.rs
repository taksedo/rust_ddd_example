use std::sync::{Arc, Mutex};

use derive_new::new;

use domain::main::cart::value_objects::customer_id::CustomerId;

use crate::main::cart::access::cart_extractor::CartExtractor;
use crate::main::cart::get_cart::{CartInfo, CartItem, GetCart, GetCartUseCaseError};
use crate::main::menu::access::meal_extractor::MealExtractor;

#[derive(new, Debug)]
pub struct GetCartUseCase {
    meal_extractor: Arc<Mutex<dyn MealExtractor>>,
    cart_extractor: Arc<Mutex<dyn CartExtractor>>,
}

impl GetCart for GetCartUseCase {
    fn execute(&self, for_customer: CustomerId) -> Result<CartInfo, GetCartUseCaseError> {
        let cart = &self
            .cart_extractor
            .lock()
            .unwrap()
            .get_cart(for_customer.clone());
        if let Some(option_value) = cart {
            let cart_item_list = option_value
                .meals
                .iter()
                .map(|(meal_id, count)| {
                    let meal = &self.meal_extractor.lock().unwrap().get_by_id(*meal_id);
                    if meal.is_none() {
                        panic!("Meal #{} not found", meal_id.to_i64())
                    }
                    CartItem {
                        meal_id: *meal_id,
                        count: *count,
                        meal_name: meal.clone().unwrap().name,
                    }
                })
                .collect();
            Ok(CartInfo::new(for_customer, cart_item_list))
        } else {
            Err(GetCartUseCaseError::CartNotFound)
        }
    }
}
