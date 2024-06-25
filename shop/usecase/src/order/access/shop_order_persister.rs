use std::fmt::Debug;

use domain::order::shop_order::ShopOrder;

pub trait ShopOrderPersister: Debug + Send {
    fn save(&mut self, order: ShopOrder);
}
