use std::sync::{Arc, Mutex};

use common::types::main::errors::error::ToError;
use derive_new::new;
use domain::main::order::{
    customer_has_active_order::CustomerHasActiveOrder,
    get_meal_price::GetMealPrice,
    shop_order::{CheckoutError, ShopOrder},
    value_objects::shop_order_id::ShopOrderIdGenerator,
};

use crate::main::{
    cart::access::cart_extractor::CartExtractor,
    order::{
        access::shop_order_persister::ShopOrderPersister,
        checkout::{Checkout, CheckoutRequest, CheckoutUseCaseError, PaymentInfo},
        providers::payment_url_provider::PaymentUrlProvider,
    },
};

#[derive(new, Debug)]
pub struct CheckoutUseCase {
    id_generator: Arc<Mutex<dyn ShopOrderIdGenerator>>,
    cart_extractor: Arc<Mutex<dyn CartExtractor>>,
    active_order: Arc<Mutex<dyn CustomerHasActiveOrder>>,
    get_meal_price: Arc<Mutex<dyn GetMealPrice>>,
    payment_url_provider: Arc<Mutex<dyn PaymentUrlProvider>>,
    shop_order_persister: Arc<Mutex<dyn ShopOrderPersister>>,
}

impl Checkout for CheckoutUseCase {
    fn execute(&self, request: CheckoutRequest) -> Result<PaymentInfo, CheckoutUseCaseError> {
        self.cart_extractor
            .lock()
            .unwrap()
            .get_cart(request.for_customer)
            .map_or(Err(CheckoutUseCaseError::CartNotFound), |cart| {
                ShopOrder::checkout(
                    cart,
                    Arc::clone(&self.id_generator) as _,
                    Arc::clone(&self.active_order) as _,
                    request.delivery_to,
                    Arc::clone(&self.get_meal_price) as _,
                )
                .map_err(|err| err.to_error())
            })
            .map(|order| {
                self.shop_order_persister
                    .lock()
                    .unwrap()
                    .save(order.clone());
                PaymentInfo {
                    order_id: order.entity_params.id,
                    price: order.total_price(),
                    payment_url: self
                        .payment_url_provider
                        .lock()
                        .unwrap()
                        .provide_url(order.entity_params.id, order.total_price()),
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
