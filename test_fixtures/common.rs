use common::types::common::count::Count;
use rand::{thread_rng, Rng};

pub fn rnd_count() -> Count {
    let value = thread_rng().gen_range(2..5000);
    let result = Count::try_from(value);
    assert!(result.is_ok());
    result.unwrap()
}
