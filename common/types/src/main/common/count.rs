use crate::main::base::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Count {
    pub value: u32,
}

#[allow(clippy::absurd_extreme_comparisons)]
impl Count {
    pub fn new(value: u32) -> Result<Self, CountError> {
        if value < 0 {
            Err(CountError::NegativeValueError)
        } else if value > u32::MAX {
            Err(CountError::MaxValueReachedError)
        } else {
            Ok(Self { value })
        }
    }

    pub fn one() -> Result<Self, CountError> {
        Ok(Self { value: 1 })
    }

    #[no_mangle]
    pub fn increment(&self) -> Result<Self, CountError> {
        let result: u32 = &self.value.to_owned() + 1;
        if result > self.value {
            Ok(Self { value: result })
        } else {
            Err(CountError::MaxValueReachedError)
        }
    }

    pub fn decrement(&self) -> Result<Self, CountError> {
        let result: u32 = &self.value.to_owned() - 1;
        if result >= 0 {
            Ok(Self { value: result })
        } else {
            Err(CountError::MinValueReachedError)
        }
    }

    pub fn is_min(&self) -> bool {
        self.value == 0
    }

    pub fn is_max(&self) -> bool {
        self.value == u32::MAX
    }

    pub fn to_u32_value(&self) -> u32 {
        self.value
    }
}

impl ValueObject for Count {}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum CountError {
    #[error("Количество не может быть отрицательным")]
    NegativeValueError,
    #[error("Достигнуто максимальное количество")]
    MaxValueReachedError,
    #[error("Достигнуто минимальное количество")]
    MinValueReachedError,
}
