use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::http::Uri;
use common::types::main::common::address::Address;
use common::types::test_fixtures::rnd_count;
use derive_new::new;
use smart_default::SmartDefault;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::meal_id::MealId;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::get_meal_price::GetMealPrice;
use domain::main::order::value_objects::shop_order_id::{ShopOrderId, ShopOrderIdGenerator};
use domain::test_fixtures::{
    rnd_address, rnd_cart, rnd_customer_id, rnd_meal, rnd_order_id, rnd_price,
};

use crate::main::order::checkout::{Checkout, CheckoutRequest};
use crate::main::order::providers::payment_url_provider::PaymentUrlProvider;
use crate::main::order::scenarios::checkout_use_case::CheckoutUseCase;
use crate::test_fixtures::{MockCartExtractor, MockCustomerHasActiveOrder, MockShopOrderPersister};

#[test]
fn order_created_successfully() {
    let meal = rnd_meal();
    let address = rnd_address();
    let count = rnd_count();
    let customer_id = rnd_customer_id();
    let mut cart = rnd_cart();
    cart.meals = HashMap::from([(meal.entity_params.id, count)]);
    cart.for_customer = customer_id;

    let id_generator = Arc::new(Mutex::new(TestShopOrderIdGenerator::default()));

    let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
    cart_extractor.lock().unwrap().cart = Some(cart.clone());

    let active_order_rule = Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false)));
    let order_persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

    let price = rnd_price();
    let get_meal_price = Arc::new(Mutex::new(MockGetMealPrice::new(price.clone())));
    let payment_url_provider = Arc::new(Mutex::new(TestPaymentUrlProvider::new()));

    let use_case = CheckoutUseCase::new(
        Arc::clone(&id_generator) as _,
        Arc::clone(&cart_extractor) as _,
        Arc::clone(&active_order_rule) as _,
        Arc::clone(&get_meal_price) as _,
        Arc::clone(&payment_url_provider) as _,
        Arc::clone(&order_persister) as _,
    );

    let checkout_request = checkout_request(address.clone(), customer_id);
    let result = use_case.execute(checkout_request);

    let order_id = id_generator.lock().unwrap().id;

    active_order_rule
        .lock()
        .unwrap()
        .verify_invoked(&cart.for_customer);
    cart_extractor
        .lock()
        .unwrap()
        .verify_invoked(&cart.for_customer);
    order_persister.lock().unwrap().verify_invoked(
        &order_id,
        &address.clone(),
        &customer_id,
        &meal.entity_params.id,
        &count,
        &price,
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.order_id, order_id);
    assert_eq!(
        result.payment_url.to_string(),
        payment_url_provider.lock().unwrap().payment_url
    );
    order_persister.lock().unwrap().verify_price(&result.price);
}

#[derive(new, SmartDefault, Debug)]
struct TestShopOrderIdGenerator {
    #[default(rnd_order_id())]
    id: ShopOrderId,
}

impl ShopOrderIdGenerator for TestShopOrderIdGenerator {
    fn generate(&self) -> ShopOrderId {
        self.id
    }
}

#[derive(new, Debug)]
struct MockGetMealPrice {
    price: Price,
}

impl GetMealPrice for MockGetMealPrice {
    fn invoke(&self, _: MealId) -> Price {
        self.price.clone()
    }
}

#[derive(new, Debug)]
struct TestPaymentUrlProvider {
    #[new(value = "\"http://localhost/\".to_string()")]
    payment_url: String,
}

impl PaymentUrlProvider for TestPaymentUrlProvider {
    fn provide_url(&self, _order_id: ShopOrderId, _price: Price) -> Uri {
        self.payment_url.as_str().parse::<Uri>().unwrap()
    }
}

fn checkout_request(address: Address, customer_id: CustomerId) -> CheckoutRequest {
    let result = Address::try_from((
        address.street_to_string().as_str(),
        address.building_to_i16(),
    ))
    .map(|addr| CheckoutRequest::new(customer_id, addr));
    if result.is_ok() {
        result.unwrap()
    } else {
        panic!("Illegal State Exception")
    }
}
