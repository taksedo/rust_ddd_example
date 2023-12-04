use domain::main::cart::value_objects::customer_id::CustomerId;
use domain::main::menu::value_objects::price::Price;
use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait OrderExporter {
    fn export_order(&self, id: ShopOrderId, customer_id: CustomerId, total_price: Price);
}
