use rand::random_range;
use types::common::Count;

pub fn rnd_count() -> Count {
    let value = random_range(2..5000);
    let result = Count::try_from(value);
    assert!(result.is_ok());
    result.unwrap()
}
