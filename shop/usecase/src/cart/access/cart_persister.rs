use std::fmt::Debug;

use async_trait::async_trait;
use domain::cart::cart::Cart;

#[async_trait]
pub trait CartPersister: Debug + Send {
    async fn save(&mut self, cart: Cart);
}
