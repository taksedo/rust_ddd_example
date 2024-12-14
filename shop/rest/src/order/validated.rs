use std::{collections::HashMap, fmt::Display, str::FromStr};

use actix_web::{web::Query, HttpRequest};
use common::{common_rest::ValidationError, types::base::AM};
use domain::order::{shop_order::ShopOrderError, value_objects::shop_order_id::ShopOrderId};

use crate::validated::Validated;

impl Validated<ShopOrderId, i64> for ShopOrderId {
    fn validated(val: i64, error_list: AM<Vec<ValidationError>>) -> Result<Self, ()> {
        Self::try_from(val).map_err(|e| match e {
            ShopOrderError::IdGenerationError => error_list
                .lock()
                .unwrap()
                .push(ValidationError::new("Wrong Shop Order Id")),
        })
    }
}

#[allow(clippy::result_unit_err)]
pub fn validate_query_string<T>(
    req: HttpRequest,
    param_name: &str,
    error_list: AM<Vec<ValidationError>>,
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
                    .lock()
                    .unwrap()
                    .push(ValidationError::new(&err.to_string()));
                Err(())
            }
            Ok(start_id) => Ok(start_id),
        }
    } else {
        error_list
            .lock()
            .unwrap()
            .push(ValidationError::new(&format!(
                "Mandatory parameter '{param_name}' in query is absent"
            )));
        Err(())
    }
}
