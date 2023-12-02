use std::sync::Mutex;

use derive_new::new;

use domain::main::cart::value_objects::cart_id::{CartId, CartIdGenerator};

#[derive(Debug, new)]
pub struct InMemoryIncrementalCartIdGenerator {
    #[new(value = "Mutex::new(0)")]
    pub counter: Mutex<i64>,
}

impl CartIdGenerator for InMemoryIncrementalCartIdGenerator {
    fn generate(&mut self) -> CartId {
        let mut cart_id = self.counter.lock().unwrap();
        *cart_id += 1;
        CartId::try_from(*cart_id).unwrap()
    }
}
