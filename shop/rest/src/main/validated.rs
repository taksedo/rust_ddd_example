use std::sync::{Arc, Mutex};

use common::common_rest::rest_responses::ValidationError;

pub trait Validated<Entity, ValueType> {
    #[allow(clippy::result_unit_err)]
    fn validated(
        val: ValueType,
        error_list: Arc<Mutex<Vec<ValidationError>>>,
    ) -> Result<Entity, ()>;
}
