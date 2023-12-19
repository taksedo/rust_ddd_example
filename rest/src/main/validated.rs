use common::common_rest::main::rest_responses::ValidationError;
use std::sync::Arc;
use std::sync::Mutex;

pub trait Validated<Entity, ValueType> {
    #[allow(clippy::result_unit_err)]
    fn validated(
        val: ValueType,
        error_list: Arc<Mutex<Vec<ValidationError>>>,
    ) -> Result<Entity, ()>;
}
