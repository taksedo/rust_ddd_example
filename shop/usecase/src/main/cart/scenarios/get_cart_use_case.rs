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
                .meals()
                .iter()
                .map(|(meal_id, count)| {
                    let meal = &self.meal_extractor.lock().unwrap().get_by_id(meal_id);
                    if meal.is_none() {
                        panic!("Meal #{} not found", meal_id.to_i64())
                    }
                    CartItem {
                        meal_id: *meal_id,
                        count: *count,
                        meal_name: meal.clone().unwrap().name().to_owned(),
                    }
                })
                .collect();
            Ok(CartInfo::new(for_customer, cart_item_list))
        } else {
            Err(GetCartUseCaseError::CartNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use common::types::test_fixtures::rnd_count;
    use domain::test_fixtures::{rnd_cart_with_customer_id_and_meals, rnd_customer_id, rnd_meal};

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockMealExtractor};

    #[test]
    fn cart_successfully_extracted() {
        let meal = rnd_meal();

        let count = rnd_count();

        let customer_id = rnd_customer_id();

        let cart =
            rnd_cart_with_customer_id_and_meals(customer_id, HashMap::from([(*meal.id(), count)]));

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(cart.clone());

        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::new()));
        meal_extractor.lock().unwrap().meal = Some(meal.clone());

        let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
        let result = use_case.execute(customer_id);

        cart_extractor
            .lock()
            .unwrap()
            .verify_invoked(cart.for_customer());
        meal_extractor
            .lock()
            .unwrap()
            .verify_invoked_get_by_id(&meal.id());
        let extracted_cart = result.unwrap();
        assert_eq!(extracted_cart.for_customer, customer_id);
        assert_eq!(
            extracted_cart.items,
            vec![CartItem::new(*meal.id(), meal.name().to_owned(), count)]
        )
    }

    #[test]
    fn cart_not_found() {
        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        let meal_extractor = Arc::new(Mutex::new(MockMealExtractor::default()));
        let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
        let customer_id = rnd_customer_id();

        let result = use_case.execute(customer_id);

        cart_extractor.lock().unwrap().verify_invoked(&customer_id);
        meal_extractor.lock().unwrap().verify_empty();
        assert_eq!(result.unwrap_err(), GetCartUseCaseError::CartNotFound);
    }
}
