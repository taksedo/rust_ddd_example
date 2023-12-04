use domain::main::order::shop_order::ShopOrder;

pub trait ShopOrderPersister {
    fn save(&mut self, order: ShopOrder);
}
