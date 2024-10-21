use common::types::common::count::Count;
use rand::{thread_rng, Rng};

<<<<<<<< HEAD:common_test_fixtures/src/types.rs
========
use common::types::common::count::Count;

>>>>>>>> ab93ecf (Move test fixtures to separate files):test_fixtures/common.rs
pub fn rnd_count() -> Count {
    let value = thread_rng().gen_range(2..5000);
    let result = Count::try_from(value);
    assert!(result.is_ok());
    result.unwrap()
}
