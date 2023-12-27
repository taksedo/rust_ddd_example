use std::fmt::Debug;

use domain::main::cart::{cart::Cart, value_objects::customer_id::CustomerId};

pub trait CartExtractor: Debug + Send {
    fn get_cart(&mut self, for_customer: CustomerId) -> Option<Cart>;
}
