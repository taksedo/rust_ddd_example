use domain::main::cart::cart::Cart;

pub trait CartRemover {
    fn delete_cart(&self, cart: Cart);
}
