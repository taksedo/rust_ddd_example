use common::types::{base::generic_types::AM, errors::error::ToError};
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
