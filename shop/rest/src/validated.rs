use common::{common_rest::ValidationError, types::base::RCell};

pub trait Validated<T> {
    fn validated(val: T, error_list: RCell<Vec<ValidationError>>) -> Option<Self>
    where
        Self: Sized;
}
