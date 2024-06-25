use std::fmt::Debug;

use crate::cart::value_objects::customer_id::CustomerId;

pub trait CustomerHasActiveOrder: Debug + Send {
    fn invoke(&mut self, for_customer: &CustomerId) -> bool;
}
