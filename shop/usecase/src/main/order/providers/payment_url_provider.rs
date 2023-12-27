use std::fmt::Debug;

use actix_web::http::Uri;
use domain::main::{
    menu::value_objects::price::Price, order::value_objects::shop_order_id::ShopOrderId,
};

pub trait PaymentUrlProvider: Debug + Send {
    fn provide_url(&self, order_id: ShopOrderId, price: Price) -> Uri;
}
