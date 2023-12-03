use crate::main::cart::value_objects::customer_id::CustomerId;

pub trait CustomerHasActiveOrder {
    fn invoke(&self, for_customer: CustomerId) -> bool;
}
