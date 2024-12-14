use common::{common_rest::ValidationError, types::base::AM};

pub trait Validated<Entity, ValueType> {
    #[allow(clippy::result_unit_err)]
    fn validated(val: ValueType, error_list: AM<Vec<ValidationError>>) -> Result<Entity, ()>;
}
