use std::{
    collections::{HashSet, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

use common::types::{
    base::{AM, DomainEntity, DomainEntityTrait, Version},
    common::{Address, Count},
};
use derive_getters::Getters;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;

use crate::{
    cart::{cart::Cart, value_objects::customer_id::CustomerId},
    menu::value_objects::{meal_id::MealId, price::Price},
    order::{
        customer_has_active_order::CustomerHasActiveOrder,
        customer_order_events::{
            ShopOrderCancelledDomainEvent, ShopOrderCompletedDomainEvent,
            ShopOrderConfirmedDomainEvent, ShopOrderCreatedDomainEvent, ShopOrderEventEnum,
            ShopOrderPaidDomainEvent,
        },
        get_meal_price::GetMealPrice,
        shop_order::OrderState::{Cancelled, Completed, Confirmed, Paid, WaitingForPayment},
        value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator},
    },
};

#[derive(new, Debug, Clone, PartialEq, Serialize, Deserialize, SmartDefault, Getters)]
pub struct ShopOrder {
    #[getter(skip)]
    pub(crate) entity_params: DomainEntity<ShopOrderId, ShopOrderEventEnum>,
    #[default(OffsetDateTime::now_utc())]
    pub(crate) created: OffsetDateTime,
    pub(crate) for_customer: CustomerId,
    pub(crate) address: Address,
    pub(crate) order_items: HashSet<OrderItem>,
    pub(crate) state: OrderState,
}

impl ShopOrder {
    pub async fn checkout(
        cart: Cart,
        id_generator: AM<dyn ShopOrderIdGenerator>,
        customer_has_active_order: AM<dyn CustomerHasActiveOrder>,
        address: Address,
        get_meal_price: AM<dyn GetMealPrice>,
    ) -> Result<ShopOrder, CheckoutError> {
        if customer_has_active_order
            .lock()
            .await
            .invoke(cart.for_customer())
            .await
        {
            return Err(CheckoutError::AlreadyHasActiveOrder);
        }
        let meals = cart.meals();
        if !meals.is_empty() {
            let mut set = HashSet::new();

            for (meal_id, count) in meals {
                let price = get_meal_price.lock().await.invoke(meal_id).await;
                set.insert(OrderItem::new(*meal_id, price, *count));
            }
            let id = id_generator.lock().await.generate();
            let mut shop_order = ShopOrder::new(
                DomainEntity::new(id, Default::default()),
                OffsetDateTime::now_utc(),
                *cart.for_customer(),
                address,
                set,
                Default::default(),
            );
            let total_price = shop_order.total_price();
            shop_order.add_event(
                ShopOrderCreatedDomainEvent::new(id, *cart.for_customer(), total_price).into(),
            );
            Ok(shop_order)
        } else {
            Err(CheckoutError::EmptyCart)
        }
    }

    pub fn confirm(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_confirmed(),
            ShopOrderConfirmedDomainEvent::new(*self.id()).into(),
        )
    }

    pub fn pay(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_paid(),
            ShopOrderPaidDomainEvent::new(*self.id()).into(),
        )
    }

    pub fn complete(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_completed(),
            ShopOrderCompletedDomainEvent::new(*self.id()).into(),
        )
    }

    pub fn cancel(&mut self) -> Result<(), InvalidState> {
        self.change_state(
            OrderState::new_cancelled(),
            ShopOrderCancelledDomainEvent::new(*self.id()).into(),
        )
    }

    pub fn change_state(
        &mut self,
        new_state: OrderState,
        event: ShopOrderEventEnum,
    ) -> Result<(), InvalidState> {
        if self.state == new_state {
            Ok(())
        } else if self.state.can_change_to(&new_state) {
            self.state = new_state;
            self.add_event(event);
            Ok(())
        } else {
            Err(InvalidState)
        }
    }
    pub fn total_price(&self) -> Price {
        self.order_items
            .iter()
            .map(|it| it.price.multiple(it.count))
            .fold(Price::zero(), |acc, it| acc.add(it))
    }

    pub fn is_active(&self) -> bool {
        match &self.state {
            Cancelled(value) => *value,
            Completed(value) => *value,
            Confirmed(value) => *value,
            Paid(value) => *value,
            WaitingForPayment(value) => *value,
        }
    }

    pub fn ready_for_confirm_or_cancel(&self) -> bool {
        matches!(&self.state, Paid(_))
    }

    pub fn id(&self) -> &ShopOrderId {
        self.entity_params.id()
    }

    pub fn version(&self) -> &Version {
        self.entity_params.version()
    }

    pub(self) fn add_event(&mut self, event: ShopOrderEventEnum) {
        self.entity_params.add_event(event)
    }

    pub fn pop_events(&mut self) -> Vec<ShopOrderEventEnum> {
        self.entity_params.pop_events()
    }
}

#[derive(new, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct OrderItem {
    pub meal_id: MealId,
    pub price: Price,
    pub count: Count,
}

impl OrderItem {
    pub fn hash_code(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.meal_id.hash(&mut state);
        state.finish()
    }
}

#[derive(new, PartialEq, Eq, Debug, Clone, Hash, SmartDefault, Serialize, Deserialize)]
pub enum OrderState {
    Cancelled(#[new(value = "false")] bool),
    Completed(#[new(value = "false")] bool),
    Confirmed(#[new(value = "true")] bool),
    Paid(#[new(value = "true")] bool),
    #[default]
    WaitingForPayment(#[new(value = "true")] bool),
}

impl OrderState {
    pub fn can_change_to(&self, state: &OrderState) -> bool {
        match self {
            Confirmed(_) => matches!(state, Completed(_)),

            Paid(_) => {
                matches!(state, Confirmed(_) | Cancelled(_))
            }

            WaitingForPayment(_) => matches!(state, Paid(_)),

            _ => false,
        }
    }

    pub fn is_active(&self) -> bool {
        match &self {
            Cancelled(value) => *value,
            Completed(value) => *value,
            Confirmed(value) => *value,
            Paid(value) => *value,
            WaitingForPayment(value) => *value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CheckoutError {
    EmptyCart,
    AlreadyHasActiveOrder,
}

#[derive(Debug, PartialEq)]
pub struct InvalidState;

#[derive(Debug, PartialEq)]
pub enum ShopOrderError {
    IdGenerationError,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use async_trait::async_trait;
    use bigdecimal::{BigDecimal, num_bigint::BigInt};
    use common::{test_fixtures::rnd_count, types::base::AMTrait};

    use super::*;
    use crate::test_fixtures::{
        order_with_state, rnd_address, rnd_cart, rnd_meal_id, rnd_order, rnd_order_id, rnd_price,
    };

    #[tokio::test]
    async fn checkout_success() {
        let id_generator = AM::new_am(MockOrderIdGenerator::default());
        let id = id_generator.lock().await.id;
        let meal_id = rnd_meal_id();
        let count = rnd_count();
        let price = rnd_price();
        let address = rnd_address();

        let get_meal_price = AM::new_am(HashMapStoragePriceProvider::default());
        get_meal_price
            .lock()
            .await
            .storage
            .insert(meal_id, price.clone());
        let mut cart = rnd_cart();
        cart.meals.insert(meal_id, count);

        let result = ShopOrder::checkout(
            cart.clone(),
            id_generator.clone(),
            AM::new_am(MockCustomerHasActiveOrder::new(false)),
            address.clone(),
            get_meal_price.clone(),
        )
        .await;

        let mut order = result.unwrap();

        assert_eq!(order.for_customer(), cart.for_customer());
        assert_eq!(
            order.order_items(),
            &HashSet::from([OrderItem::new(meal_id, price, count)])
        );
        assert_eq!(order.id(), &id);
        assert_eq!(order.address(), &address);
        assert!(matches!(order.state(), WaitingForPayment(_)));
        let events: Vec<ShopOrderCreatedDomainEvent> = order
            .pop_events()
            .iter()
            .map(|it| it.clone().try_into().unwrap())
            .collect();
        assert_eq!(events.len(), 1);
        let event = events.first().unwrap().clone();
        assert_eq!(event.order_id, id);
        assert_eq!(&event.for_customer, cart.for_customer());
        assert_eq!(event.total_price, order.total_price());
    }

    #[tokio::test]
    async fn checkout_already_has_active_user() {
        let id_generator = AM::new_am(MockOrderIdGenerator::default());
        let meal_id = rnd_meal_id();
        let count = rnd_count();
        let price = rnd_price();
        let address = rnd_address();

        let meal_price_only_for_special_meal = AM::new_am(HashMapStoragePriceProvider::default());
        meal_price_only_for_special_meal
            .lock()
            .await
            .storage
            .insert(meal_id, price);

        let mut cart = rnd_cart();
        cart.meals.insert(meal_id, count);

        let result = ShopOrder::checkout(
            cart.clone(),
            id_generator.clone(),
            AM::new_am(MockCustomerHasActiveOrder::new(true)),
            address.clone(),
            meal_price_only_for_special_meal.clone(),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CheckoutError::AlreadyHasActiveOrder);
    }

    #[tokio::test]
    async fn checkout_empty_cart() {
        let id_generator = AM::new_am(MockOrderIdGenerator::default());
        let cart = rnd_cart();
        let get_meal_price = AM::new_am(HashMapStoragePriceProvider::default());
        get_meal_price
            .lock()
            .await
            .storage
            .insert(rnd_meal_id(), rnd_price());
        let result = ShopOrder::checkout(
            cart.clone(),
            id_generator.clone(),
            AM::new_am(MockCustomerHasActiveOrder::new(false)),
            rnd_address(),
            get_meal_price.clone(),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CheckoutError::EmptyCart)
    }

    #[test]
    fn active_true() {
        let states = [
            OrderState::new_waiting_for_payment(),
            OrderState::new_confirmed(),
            OrderState::new_paid(),
        ];

        states.iter().for_each(|it| {
            dbg!(&it);
            assert!(it.is_active())
        });
    }

    #[test]
    fn active_false() {
        let states = [OrderState::new_completed(), OrderState::new_cancelled()];

        states.iter().for_each(|it| assert!(!it.is_active()));
    }

    #[test]
    fn complete_order_success() {
        let mut order = order_with_state(OrderState::new_waiting_for_payment());
        assert!(order.pay().is_ok());
        assert!(matches!(order.state(), Paid(_)));
        let event: Vec<ShopOrderPaidDomainEvent> = order
            .pop_events()
            .iter()
            .map(|it| it.clone().try_into().unwrap())
            .collect();
        assert_eq!(event.len(), 1);
        assert_eq!(&event.first().unwrap().order_id, order.id());
    }

    #[test]
    fn complete_order_already() {
        let mut order = order_with_state(OrderState::new_completed());
        assert!(order.complete().is_ok());
        assert!(matches!(order.state(), Completed(_)));
        assert!(order.pop_events().is_empty());
    }

    #[test]
    fn complete_order_invalid_state() {
        let states = [
            OrderState::new_waiting_for_payment(),
            OrderState::new_paid(),
            OrderState::new_cancelled(),
        ];

        states.iter().for_each(|state| {
            let mut order = order_with_state(state.clone());
            assert_eq!(order.complete().unwrap_err(), InvalidState);
            assert_eq!(order.state(), state);
            assert!(order.pop_events().is_empty())
        });
    }

    #[test]
    fn pay_order_success() {
        let mut order = order_with_state(OrderState::new_waiting_for_payment());
        assert!(order.pay().is_ok());
        assert!(matches!(order.state(), Paid(_)));
        let event: Vec<ShopOrderPaidDomainEvent> = order
            .pop_events()
            .iter()
            .map(|it| it.clone().try_into().unwrap())
            .collect();
        assert_eq!(event.len(), 1);
        assert_eq!(&event.first().unwrap().order_id, order.id());
    }

    #[test]
    fn pay_order_already() {
        let mut order = order_with_state(OrderState::new_paid());
        assert!(order.pay().is_ok());
        assert!(matches!(order.state(), Paid(_)));
        assert!(order.pop_events().is_empty());
    }

    #[test]
    fn pay_order_invalid_state() {
        let states = [
            OrderState::new_confirmed(),
            OrderState::new_completed(),
            OrderState::new_cancelled(),
        ];

        states.iter().for_each(|state| {
            let mut order = order_with_state(state.clone());
            assert_eq!(order.pay().unwrap_err(), InvalidState);
            assert_eq!(order.state(), state);
            assert!(order.pop_events().is_empty())
        });
    }

    #[test]
    fn order_is_ready_to_confirm_or_cancel() {
        let order = order_with_state(OrderState::new_paid());
        assert!(order.ready_for_confirm_or_cancel());
    }

    #[test]
    fn order_cannot_be_cancelled() {
        let states = [
            OrderState::new_confirmed(),
            OrderState::new_completed(),
            OrderState::new_waiting_for_payment(),
            OrderState::new_cancelled(),
        ];

        states.iter().for_each(|state| {
            let order = order_with_state(state.clone());
            assert!(!order.ready_for_confirm_or_cancel());
        });
    }

    #[test]
    fn cancel_order_success() {
        let mut order = order_with_state(OrderState::new_paid());
        assert!(order.cancel().is_ok());
        assert!(matches!(order.state(), Cancelled(_)));
        let event: Vec<ShopOrderCancelledDomainEvent> = order
            .pop_events()
            .iter()
            .map(|it| it.clone().try_into().unwrap())
            .collect();
        assert_eq!(event.len(), 1);
        assert_eq!(&event.first().unwrap().order_id, order.id());
    }

    #[test]
    fn cancel_order_already() {
        let mut order = order_with_state(OrderState::new_cancelled());
        assert!(order.cancel().is_ok());
        assert!(matches!(order.state(), Cancelled(_)));
        assert!(order.pop_events().is_empty());
    }

    #[test]
    fn cancel_order_invalid_state() {
        let states = [
            OrderState::new_confirmed(),
            OrderState::new_completed(),
            OrderState::new_waiting_for_payment(),
        ];

        states.iter().for_each(|state| {
            let mut order = order_with_state(state.clone());
            assert_eq!(order.cancel().unwrap_err(), InvalidState);
            assert_eq!(order.state(), state);
            assert!(order.pop_events().is_empty())
        });
    }

    #[test]
    fn confirm_order_success() {
        let mut order = order_with_state(OrderState::new_paid());
        assert!(order.confirm().is_ok());
        assert!(matches!(order.state(), Confirmed(_)));
        let event: Vec<ShopOrderConfirmedDomainEvent> = order
            .pop_events()
            .iter()
            .map(|it| it.clone().try_into().unwrap())
            .collect();
        assert_eq!(event.len(), 1);
        assert_eq!(&event.first().unwrap().order_id, order.id());
    }

    #[test]
    fn confirm_order_already() {
        let mut order = order_with_state(OrderState::new_confirmed());
        assert!(order.confirm().is_ok());
        assert!(matches!(order.state(), Confirmed(_)));
        assert!(order.pop_events().is_empty());
    }

    #[test]
    fn confirm_order_invalid_state() {
        let states = [
            OrderState::new_cancelled(),
            OrderState::new_completed(),
            OrderState::new_waiting_for_payment(),
        ];

        states.iter().for_each(|state| {
            let mut order = order_with_state(state.clone());
            assert_eq!(order.confirm().unwrap_err(), InvalidState);
            assert_eq!(order.state(), state);
            assert!(order.pop_events().is_empty())
        });
    }

    #[test]
    fn calculate_total() {
        let order_item_1 = OrderItem::new(
            rnd_meal_id(),
            Price::try_from(BigDecimal::from_str("1.03").unwrap()).unwrap(),
            Count::try_from(2).unwrap(),
        );
        let order_item_2 = OrderItem::new(
            rnd_meal_id(),
            Price::try_from(BigDecimal::from_str("91.33").unwrap()).unwrap(),
            Count::try_from(4).unwrap(),
        );

        let order = rnd_order(HashSet::from([order_item_1, order_item_2]));
        assert_eq!(
            order.total_price(),
            Price::try_from(BigDecimal::new(BigInt::from(36738), 2)).unwrap()
        )
    }

    #[derive(new, Default, Debug)]
    struct HashMapStoragePriceProvider {
        storage: HashMap<MealId, Price>,
    }

    #[async_trait]
    impl GetMealPrice for HashMapStoragePriceProvider {
        async fn invoke(&self, for_meal_id: &MealId) -> Price {
            let result = &self.storage.get(for_meal_id);
            result.unwrap().clone()
        }
    }

    #[derive(SmartDefault, Debug)]
    struct MockOrderIdGenerator {
        #[default(rnd_order_id())]
        id: ShopOrderId,
    }

    impl ShopOrderIdGenerator for MockOrderIdGenerator {
        fn generate(&mut self) -> ShopOrderId {
            self.id
        }
    }

    #[derive(new, Debug)]
    struct MockCustomerHasActiveOrder {
        status: bool,
    }

    #[async_trait]
    impl CustomerHasActiveOrder for MockCustomerHasActiveOrder {
        async fn invoke(&mut self, _for_customer: &CustomerId) -> bool {
            self.status
        }
    }
}
