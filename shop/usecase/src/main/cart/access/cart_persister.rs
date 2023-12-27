use std::fmt::Debug;

use domain::main::cart::cart::Cart;

pub trait CartPersister: Debug + Send {
    fn save(&mut self, cart: Cart);
}
