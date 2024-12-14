use serde_derive::{Deserialize, Serialize};

use crate::base::ValueObject;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Count(i32);

#[allow(clippy::absurd_extreme_comparisons)]
impl Count {
    pub fn one() -> Self {
        Self(1)
    }

    #[no_mangle]
    pub fn increment(&self) -> Result<Self, CountError> {
        match self.0 {
            i32::MAX => Err(CountError::MaxValueReachedError),
            _ => Ok(Self(&self.0 + 1)),
        }
    }

    pub fn decrement(&self) -> Result<Self, CountError> {
        match &self.0 {
            0 => Err(CountError::MinValueReachedError),
            _ => Ok(Self(&self.0 - 1)),
        }
    }

    pub fn is_min(&self) -> bool {
        self.0 == 0
    }

    pub fn is_max(&self) -> bool {
        self.0 == i32::MAX
    }

    pub fn to_i32(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Count {
    type Error = CountError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            #[allow(clippy::absurd_extreme_comparisons)]
            _ if value > i32::MAX => Err(CountError::MaxValueReachedError),
            _ if value < 0 => Err(CountError::NegativeValueError),
            _ => Ok(Self(value)),
        }
    }
}

impl ValueObject for Count {}

#[derive(Debug, PartialEq, Clone)]
pub enum CountError {
    NegativeValueError,
    MaxValueReachedError,
    MinValueReachedError,
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn create_count_success(#[values(0_i32, 1_i32, i32::MAX)] value: i32) {
        let result = Count::try_from(value);
        let count = result.unwrap();
        assert_eq!(count.to_i32(), value);
    }

    #[test]
    fn create_count_one() {
        let result = Count::one();
        assert_eq!(result.to_i32(), 1)
    }

    #[test]
    fn create_count_negative_value() {
        let result = Count::try_from(-1);
        assert_eq!(result, Err(CountError::NegativeValueError));
    }

    #[test]
    fn increment_success() {
        let count = Count::try_from(1).unwrap();
        let increment = count.increment();
        assert_eq!(increment, Count::try_from(count.to_i32() + 1));
    }

    #[test]
    fn increment_max_value_reached() {
        let count = Count::try_from(i32::MAX).unwrap();
        let result = count.increment();
        assert_eq!(result, Err(CountError::MaxValueReachedError));
    }

    #[rstest]
    fn decrement_success(#[values(1_i32, i32::MAX)] value: i32) {
        let count = Count::try_from(value).unwrap();
        let increment = count.decrement();
        assert_eq!(increment, Count::try_from(count.to_i32() - 1));
    }

    #[test]
    fn decrement_min_value_reached() {
        let count = Count::try_from(0).unwrap();
        let result = count.decrement();
        assert_eq!(result, Err(CountError::MinValueReachedError));
    }

    #[test]
    fn check_is_min_value_true() {
        let count = Count::try_from(0).unwrap();
        assert!(count.is_min());
    }

    #[test]
    fn check_is_min_value_false() {
        let count = Count::try_from(1).unwrap();
        assert!(!count.is_min());
    }

    #[test]
    fn check_is_max_value_true() {
        let count = Count::try_from(i32::MAX).unwrap();
        assert!(count.is_max());
    }

    #[test]
    fn check_is_max_value_false() {
        let count = Count::try_from(1).unwrap();
        assert!(!count.is_max());
    }
}
