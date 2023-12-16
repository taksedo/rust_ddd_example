use std::fmt::Debug;

use domain::main::{
    cart::value_objects::customer_id::CustomerId, menu::value_objects::price::Price,
    order::value_objects::shop_order_id::ShopOrderId,
};

pub trait OrderExporter: Debug + Send {
    fn export_order(&mut self, id: ShopOrderId, customer_id: CustomerId, total_price: Price);
}
