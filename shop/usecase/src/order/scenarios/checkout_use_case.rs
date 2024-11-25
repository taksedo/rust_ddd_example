use common::types::{base::generic_types::AM, errors::error::ToError};
use derive_new::new;
use domain::order::{
    customer_has_active_order::CustomerHasActiveOrder,
    get_meal_price::GetMealPrice,
    shop_order::{CheckoutError, ShopOrder},
    value_objects::shop_order_id::ShopOrderIdGenerator,
};

use crate::{
    cart::access::cart_extractor::CartExtractor,
    order::{
        access::shop_order_persister::ShopOrderPersister,
        checkout::{Checkout, CheckoutRequest, CheckoutUseCaseError, PaymentInfo},
        providers::payment_url_provider::PaymentUrlProvider,
    },
};

#[derive(new, Debug)]
pub struct CheckoutUseCase<
    ShOIdGenerator,
    CExtractor,
    CustomerHasActiveO,
    GetMPrice,
    PaymUrlProvider,
    ShOPersister,
> where
    ShOIdGenerator: ShopOrderIdGenerator,
    CExtractor: CartExtractor,
    CustomerHasActiveO: CustomerHasActiveOrder,
    GetMPrice: GetMealPrice,
    PaymUrlProvider: PaymentUrlProvider,
    ShOPersister: ShopOrderPersister,
{
    id_generator: AM<ShOIdGenerator>,
    cart_extractor: AM<CExtractor>,
    active_order: AM<CustomerHasActiveO>,
    get_meal_price: AM<GetMPrice>,
    payment_url_provider: AM<PaymUrlProvider>,
    shop_order_persister: AM<ShOPersister>,
}

impl<ShOIdGenerator, CExtractor, CustomerHasActiveO, GetMPrice, PaymUrlProvider, ShOPersister>
    Checkout
    for CheckoutUseCase<
        ShOIdGenerator,
        CExtractor,
        CustomerHasActiveO,
        GetMPrice,
        PaymUrlProvider,
        ShOPersister,
    >
where
    ShOIdGenerator: ShopOrderIdGenerator + 'static,
    CExtractor: CartExtractor,
    CustomerHasActiveO: CustomerHasActiveOrder + 'static,
    GetMPrice: GetMealPrice + 'static,
    PaymUrlProvider: PaymentUrlProvider,
    ShOPersister: ShopOrderPersister,
{
    fn execute(&self, request: &CheckoutRequest) -> Result<PaymentInfo, CheckoutUseCaseError> {
        self.cart_extractor
            .lock()
            .unwrap()
            .get_cart(&request.for_customer)
            .map_or(Err(CheckoutUseCaseError::CartNotFound), |cart| {
                ShopOrder::checkout(
                    cart,
                    self.id_generator.clone(),
                    self.active_order.clone(),
                    request.delivery_to.clone(),
                    self.get_meal_price.clone(),
                )
                .map_err(|err| err.to_error())
            })
            .map(|order| {
                self.shop_order_persister
                    .lock()
                    .unwrap()
                    .save(order.clone());
                PaymentInfo {
                    order_id: *order.id(),
                    price: order.total_price(),
                    payment_url: self
                        .payment_url_provider
                        .lock()
                        .unwrap()
                        .provide_url(order.id(), order.total_price()),
                }
            })
    }
}

impl ToError<CheckoutUseCaseError> for CheckoutError {
    fn to_error(self) -> CheckoutUseCaseError {
        match self {
            CheckoutError::AlreadyHasActiveOrder => CheckoutUseCaseError::AlreadyHasActiveOrder,
            CheckoutError::EmptyCart => CheckoutUseCaseError::EmptyCart,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use actix_web::http::Uri;
    use common::types::common::address::Address;
    use domain::{
        cart::value_objects::customer_id::CustomerId,
        menu::value_objects::{meal_id::MealId, price::Price},
        order::value_objects::shop_order_id::ShopOrderId,
    };
    use smart_default::SmartDefault;

    use super::*;
    use crate::{
        common_test_fixtures::rnd_count,
        domain_test_fixtures::{
            rnd_address, rnd_cart, rnd_cart_with_customer_id_and_meals, rnd_customer_id, rnd_meal,
            rnd_order_id, rnd_price,
        },
        test_fixtures::{MockCartExtractor, MockCustomerHasActiveOrder, MockShopOrderPersister},
    };

    #[test]
    fn order_created_successfully() {
        let meal = rnd_meal();
        let address = rnd_address();
        let count = rnd_count();
        let customer_id = rnd_customer_id();
        let cart =
            rnd_cart_with_customer_id_and_meals(customer_id, HashMap::from([(*meal.id(), count)]));

        let id_generator = Arc::new(Mutex::new(TestShopOrderIdGenerator::default()));

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(cart.clone());

        let active_order_rule = Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false)));
        let order_persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let price = rnd_price();
        let get_meal_price = Arc::new(Mutex::new(MockGetMealPrice::new(price.clone())));
        let payment_url_provider = Arc::new(Mutex::new(TestPaymentUrlProvider::new()));

        let use_case = CheckoutUseCase::new(
            id_generator.clone(),
            cart_extractor.clone(),
            active_order_rule.clone(),
            get_meal_price.clone(),
            payment_url_provider.clone(),
            order_persister.clone(),
        );

        let checkout_request = checkout_request(address.clone(), customer_id);
        let result = use_case.execute(&checkout_request);

        let order_id = id_generator.lock().unwrap().id;

        active_order_rule
            .lock()
            .unwrap()
            .verify_invoked(cart.for_customer());
        cart_extractor
            .lock()
            .unwrap()
            .verify_invoked(cart.for_customer());
        order_persister.lock().unwrap().verify_invoked(
            &order_id,
            &address,
            &customer_id,
            meal.id(),
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

    #[test]
    fn cart_not_found() {
        let id_generator = Arc::new(Mutex::new(TestShopOrderIdGenerator::default()));
        let active_order_rule = Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false)));

        let order_persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));
        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));

        let get_meal_price = Arc::new(Mutex::new(MockGetMealPrice::default()));
        let payment_url_provider = Arc::new(Mutex::new(TestPaymentUrlProvider::new()));

        let use_case = CheckoutUseCase::new(
            id_generator.clone(),
            cart_extractor.clone(),
            active_order_rule.clone(),
            get_meal_price.clone(),
            payment_url_provider.clone(),
            order_persister.clone(),
        );

        let checkout_request = checkout_request(rnd_address(), rnd_customer_id());
        let result = use_case.execute(&checkout_request);

        order_persister.lock().unwrap().verify_empty();
        active_order_rule.lock().unwrap().verify_empty();
        cart_extractor
            .lock()
            .unwrap()
            .verify_invoked(&checkout_request.for_customer);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CheckoutUseCaseError::CartNotFound);
    }

    #[test]
    fn cart_is_empty() {
        let cart = rnd_cart();
        let customer_id = cart.for_customer();

        let id_generator = Arc::new(Mutex::new(TestShopOrderIdGenerator::default()));

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(cart.clone());

        let active_order_rule = Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(false)));
        let order_persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let price = rnd_price();
        let get_meal_price = Arc::new(Mutex::new(MockGetMealPrice::new(price.clone())));
        let payment_url_provider = Arc::new(Mutex::new(TestPaymentUrlProvider::new()));

        let use_case = CheckoutUseCase::new(
            id_generator.clone(),
            cart_extractor.clone(),
            active_order_rule.clone(),
            get_meal_price.clone(),
            payment_url_provider.clone(),
            order_persister.clone(),
        );

        let checkout_request = checkout_request(rnd_address(), *customer_id);
        let result = use_case.execute(&checkout_request);

        order_persister.lock().unwrap().verify_empty();
        active_order_rule
            .lock()
            .unwrap()
            .verify_invoked(&checkout_request.for_customer);
        cart_extractor
            .lock()
            .unwrap()
            .verify_invoked(&checkout_request.for_customer);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CheckoutUseCaseError::EmptyCart);
    }

    #[test]
    fn already_has_active_order() {
        let cart = rnd_cart();

        let id_generator = Arc::new(Mutex::new(TestShopOrderIdGenerator::default()));

        let cart_extractor = Arc::new(Mutex::new(MockCartExtractor::default()));
        cart_extractor.lock().unwrap().cart = Some(cart.clone());

        let active_order_rule = Arc::new(Mutex::new(MockCustomerHasActiveOrder::new(true)));
        let order_persister = Arc::new(Mutex::new(MockShopOrderPersister::default()));

        let get_meal_price = Arc::new(Mutex::new(MockGetMealPrice::default()));
        let payment_url_provider = Arc::new(Mutex::new(TestPaymentUrlProvider::new()));

        let use_case = CheckoutUseCase::new(
            id_generator.clone(),
            cart_extractor.clone(),
            active_order_rule.clone(),
            get_meal_price.clone(),
            payment_url_provider.clone(),
            order_persister.clone(),
        );

        order_persister.lock().unwrap().verify_empty();
        cart_extractor.lock().unwrap().verify_empty();
        active_order_rule.lock().unwrap().verify_empty();
        let result = use_case.execute(&checkout_request(rnd_address(), *cart.for_customer()));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CheckoutUseCaseError::AlreadyHasActiveOrder
        );
    }

    #[derive(new, SmartDefault, Debug)]
    struct TestShopOrderIdGenerator {
        #[default(rnd_order_id())]
        id: ShopOrderId,
    }

    impl ShopOrderIdGenerator for TestShopOrderIdGenerator {
        fn generate(&mut self) -> ShopOrderId {
            self.id
        }
    }

    #[derive(new, Debug, Default)]
    struct MockGetMealPrice {
        price: Price,
    }

    impl GetMealPrice for MockGetMealPrice {
        fn invoke(&self, _: &MealId) -> Price {
            self.price.clone()
        }
    }

    #[derive(new, Debug)]
    struct TestPaymentUrlProvider {
        #[new(value = "\"http://localhost/\".to_string()")]
        payment_url: String,
    }

    impl PaymentUrlProvider for TestPaymentUrlProvider {
        fn provide_url(&self, _order_id: &ShopOrderId, _price: Price) -> Uri {
            self.payment_url.as_str().parse::<Uri>().unwrap()
        }
    }

    fn checkout_request(address: Address, customer_id: CustomerId) -> CheckoutRequest {
        match Address::try_from((
            address.street_to_string().as_str(),
            address.building_to_i16(),
        ))
        .map(|addr| CheckoutRequest::new(customer_id, addr))
        {
            Ok(request) => request,
            Err(_) => panic!("Illegal State Exception"),
        }
    }
}
