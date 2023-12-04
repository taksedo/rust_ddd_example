use actix_web::dev::Url;
use common::types::main::common::address::Address;
use thiserror::Error;

use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait Checkout {
    fn execute(&self, request: CheckoutRequest) -> Result<PaymentInfo, CheckoutUseCaseError>;
}

pub struct PaymentInfo {
    pub order_id: ShopOrderId,
    pub price: Price,
    pub payment_url: Url,
}

pub struct CheckoutRequest {
    pub for_customer: CustomerId,
    pub delivery_to: Address,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CheckoutUseCaseError {
    #[error("Cart not found")]
    CartNotFound,
    #[error("Empty cart")]
    EmptyCart,
    #[error("Already has active order")]
    AlreadyHasActiveOrder,
    #[error("TODO")]
    InvalidAddress,
}
