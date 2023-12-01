use std::fmt::Debug;

use domain::main::cart::cart::Cart;

pub trait CartRemover: Debug + Send {
    fn delete_cart(&mut self, cart: Cart);
}
