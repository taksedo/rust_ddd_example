use crate::main::base::value_object::ValueObject;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Count {
    pub value: i32,
}

#[allow(clippy::absurd_extreme_comparisons)]
impl Count {
    pub fn from(value: i32) -> Result<Self, CountError> {
        match value {
            _ if value > i32::MAX => Err(CountError::MaxValueReachedError),
            _ if value < 0 => Err(CountError::NegativeValueError),
            _ => Ok(Self { value }),
        }
    }

    pub fn one() -> Self {
        Self { value: 1 }
    }

    #[no_mangle]
    pub fn increment(&self) -> Result<Self, CountError> {
        match self.value {
            i32::MAX => Err(CountError::MaxValueReachedError),
            _ => Ok(Self {
                value: &self.value + 1,
            }),
        }
    }

    pub fn decrement(&self) -> Result<Self, CountError> {
        match &self.value {
            0 => Err(CountError::MinValueReachedError),
            _ => Ok(Self {
                value: &self.value - 1,
            }),
        }
    }

    pub fn is_min(&self) -> bool {
        self.value == 0
    }

    pub fn is_max(&self) -> bool {
        self.value == i32::MAX
    }

    pub fn to_i32(&self) -> i32 {
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
