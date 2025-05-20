use std::fmt::Debug;

use async_trait::async_trait;
use domain::order::shop_order::ShopOrder;

#[async_trait]
pub trait ShopOrderPersister: Debug + Send {
    async fn save(&mut self, order: ShopOrder);
}
