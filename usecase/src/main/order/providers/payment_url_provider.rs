use actix_web::dev::Url;

use domain::main::menu::value_objects::price::Price;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait PaymentUrlProvider {
    fn provide_url(&self, order_id: ShopOrderId, price: Price) -> Url;
}
