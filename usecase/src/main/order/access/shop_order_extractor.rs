use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::order::shop_order::ShopOrder;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait ShopOrderExtractor {
    fn get_by_id(&self, order_id: ShopOrderId) -> Option<ShopOrder>;
    fn get_last_order(&self, for_customer: CustomerId) -> Option<ShopOrder>;
    fn get_all(&self, start_id: ShopOrderId, limit: i32) -> Vec<ShopOrder>;
}
