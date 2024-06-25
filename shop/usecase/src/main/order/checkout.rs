use actix_web::http::Uri;
use common::types::common::address::Address;
use derive_new::new;
use domain::{
    cart::value_objects::customer_id::CustomerId, menu::value_objects::price::Price,
    order::value_objects::shop_order_id::ShopOrderId,
};
use thiserror::Error;

pub trait Checkout {
    fn execute(&self, request: &CheckoutRequest) -> Result<PaymentInfo, CheckoutUseCaseError>;
}

#[derive(Debug)]
pub struct PaymentInfo {
    pub order_id: ShopOrderId,
    pub price: Price,
    pub payment_url: Uri,
}

#[derive(new, Debug, Clone)]
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
