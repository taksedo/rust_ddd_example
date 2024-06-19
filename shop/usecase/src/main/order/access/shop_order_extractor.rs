use std::fmt::Debug;

use domain::main::{
    cart::value_objects::customer_id::CustomerId,
    order::{shop_order::ShopOrder, value_objects::shop_order_id::ShopOrderId},
};

pub trait ShopOrderExtractor: Debug + Send {
    fn get_by_id(&mut self, order_id: &ShopOrderId) -> Option<ShopOrder>;
    fn get_last_order(&mut self, for_customer: &CustomerId) -> Option<ShopOrder>;
    fn get_all(&mut self, start_id: &ShopOrderId, limit: usize) -> Vec<ShopOrder>;
}
