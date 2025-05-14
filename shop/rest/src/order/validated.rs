use std::{collections::HashMap, fmt::Display, str::FromStr};

use actix_web::{HttpRequest, web::Query};
use common::{common_rest::ValidationError, types::base::RCell};
use domain::order::{shop_order::ShopOrderError, value_objects::shop_order_id::ShopOrderId};

use crate::validated::Validated;

impl Validated<i64> for ShopOrderId {
    fn validated(val: i64, error_list: RCell<Vec<ValidationError>>) -> Option<Self> {
        match Self::try_from(val) {
            Ok(id) => Some(id),
            Err(ShopOrderError::IdGenerationError) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new("Wrong Shop Order Id"));
                None
            }
        }
    }
}

#[allow(clippy::result_unit_err)]
pub fn validate_query_string<T>(
    req: HttpRequest,
    param_name: &str,
    error_list: RCell<Vec<ValidationError>>,
) -> Result<T, ()>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let query_params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    if let Some(val) = query_params.get(param_name) {
        match val.parse::<T>() {
            Err(err) => {
                error_list
                    .borrow_mut()
                    .push(ValidationError::new(&err.to_string()));
                Err(())
            }
            Ok(start_id) => Ok(start_id),
        }
    } else {
        error_list.borrow_mut().push(ValidationError::new(&format!(
            "Mandatory parameter '{param_name}' in query is absent"
        )));
        Err(())
    }
}
