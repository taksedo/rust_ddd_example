use async_trait::async_trait;
use common::types::base::AM;
use derive_new::new;
use domain::cart::value_objects::customer_id::CustomerId;

use crate::{
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

#[async_trait]
impl<MExtractor, CExtractor> GetCart for GetCartUseCase<MExtractor, CExtractor>
where
    MExtractor: MealExtractor,
    CExtractor: CartExtractor,
{
    async fn execute(&self, for_customer: CustomerId) -> Result<CartInfo, GetCartUseCaseError> {
        let cart = self
            .cart_extractor
            .lock()
            .await
            .get_cart(&for_customer)
            .await;
        let cart = match cart {
            Some(cart) => cart,
            None => return Err(GetCartUseCaseError::CartNotFound),
        };

        let mut meal_extractor = self.meal_extractor.lock().await;
        let cart_item_list: Vec<CartItem> = cart
            .meals()
            .iter()
            .map(|(meal_id, count)| {
                let meal = meal_extractor
                    .get_by_id(meal_id)
                    .unwrap_or_else(|| panic!("Meal #{} not found", meal_id.to_i64()));
                CartItem {
                    meal_id: *meal_id,
                    count: *count,
                    meal_name: meal.name().to_owned(),
                }
            })
            .collect();

        Ok(CartInfo::new(for_customer, cart_item_list))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use common::{test_fixtures::*, types::base::AMTrait};
    use domain::test_fixtures::*;

    use super::*;
    use crate::test_fixtures::{MockCartExtractor, MockMealExtractor};

    #[tokio::test]
    async fn cart_successfully_extracted() {
        let meal = rnd_meal();

        let count = rnd_count();

        let customer_id = rnd_customer_id();

        let cart =
            rnd_cart_with_customer_id_and_meals(customer_id, HashMap::from([(*meal.id(), count)]));

        let cart_extractor = AM::new_am(MockCartExtractor::default());
        cart_extractor.lock().await.cart = Some(cart.clone());

        let meal_extractor = AM::new_am(MockMealExtractor::new());
        meal_extractor.lock().await.meal = Some(meal.clone());

        let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
        let result = use_case.execute(customer_id).await;

        cart_extractor
            .lock()
            .await
            .verify_invoked(cart.for_customer());
        meal_extractor
            .lock()
            .await
            .verify_invoked_get_by_id(meal.id());
        let extracted_cart = result.unwrap();
        assert_eq!(extracted_cart.for_customer, customer_id);
        assert_eq!(
            extracted_cart.items,
            vec![CartItem::new(*meal.id(), meal.name().to_owned(), count)]
        )
    }

    #[tokio::test]
    async fn cart_not_found() {
        let cart_extractor = AM::new_am(MockCartExtractor::default());
        let meal_extractor = AM::new_am(MockMealExtractor::default());
        let use_case = GetCartUseCase::new(meal_extractor.clone(), cart_extractor.clone());
        let customer_id = rnd_customer_id();

        let result = use_case.execute(customer_id).await;

        cart_extractor.lock().await.verify_invoked(&customer_id);
        meal_extractor.lock().await.verify_empty();
        assert_eq!(result.unwrap_err(), GetCartUseCaseError::CartNotFound);
    }
}
