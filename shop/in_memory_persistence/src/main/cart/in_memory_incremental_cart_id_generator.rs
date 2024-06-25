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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_incremented() {
        let mut id_generator = InMemoryIncrementalCartIdGenerator::new();
        let cart_id1 = id_generator.generate();
        let cart_id2 = id_generator.generate();
        assert_eq!(cart_id1.to_i64(), cart_id2.to_i64() - 1);
    }
}
