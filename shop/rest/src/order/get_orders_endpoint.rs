use std::fmt::Debug;

use actix_web::{HttpRequest, HttpResponse, http::header::ContentType, web};
use common::{
    common_rest::{
        CursorPagedModel, GenericErrorResponse, ValidationError, to_invalid_param_bad_request,
    },
    types::base::{AM, ArcMutexTrait, RCell, RcRefCellTrait},
};
use domain::order::value_objects::shop_order_id::ShopOrderId;
use usecase::order::{GetOrders, GetOrdersUseCaseError};

use super::{
    order_model::{OrderModel, ToModel},
    validated::validate_query_string,
};
use crate::{endpoint_url::API_V1_ORDER_GET_ALL, to_error::ToRestError, validated::Validated};

/// Get orders with pagination
#[utoipa::path(
    get,
    path = API_V1_ORDER_GET_ALL,
    tag = "Order",
    responses(
        (
            status = OK,
            body = Vec<OrderModel>,
            description = "OK" 
        ),
        (
            status = BAD_REQUEST,
            description = "Bad request",
            body = GenericErrorResponse,
            example = json!(
                {
                    "type": "http://0.0.0.0:8080/bad_request",
                    "title": "Bad request",
                    "status": 400,
                    "invalid_params": 
                    [
                        {"message": "Mandatory parameter 'startId' in query is absent"},
                        {"message": "Mandatory parameter 'limit' in query is absent"}
                    ]
                }
            )
        ),
    ),
    params(
        ("limit" = usize, Query, description = "Pagination limit"),
        ("startId" = i64, Query, description = "Pagination start ID")
    )
)]
pub async fn get_orders_endpoint<T: GetOrders + Send + Debug>(
    shared_state: web::Data<AM<T>>,
    req: HttpRequest,
) -> HttpResponse {
    let error_list = RCell::new_rc(vec![]);

    match (
        match validate_query_string::<i64>(req.clone(), "startId", error_list.clone()) {
            Ok(id) => ShopOrderId::validated(id, error_list.clone()),
            Err(_) => Err(()),
        },
        validate_query_string::<usize>(req, "limit", error_list.clone()),
    ) {
        (Ok(start_id), Ok(limit)) => match shared_state.lock_un().execute(&start_id, limit + 1) {
            Ok(order_details_list) => {
                let list: Vec<OrderModel> = order_details_list
                    .into_iter()
                    .map(|it| it.to_model())
                    .collect();
                let model = if list.len() > limit {
                    let next_id = list[limit].id;
                    CursorPagedModel::new(list[..limit].to_vec(), Some(next_id))
                } else {
                    CursorPagedModel::new(list, Option::<i64>::None)
                };
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&model).unwrap())
            }
            Err(e) => e.to_rest_error(),
        },
        (_, _) => to_invalid_param_bad_request(error_list),
    }
}

impl ToRestError for GetOrdersUseCaseError {
    fn to_rest_error(self) -> HttpResponse {
        match self {
            GetOrdersUseCaseError::LimitExceed(max_size) => {
                let error_list = RCell::new_rc(vec![]);
                error_list.borrow_mut().push(ValidationError::new(&format!(
                    "Max limit is {}",
                    max_size - 1
                )));
                to_invalid_param_bad_request(error_list)
            }
        }
    }
}

pub fn get_orders_endpoint_config<T>(cfg: &mut web::ServiceConfig)
where
    T: GetOrders + 'static,
{
    cfg.route(
        API_V1_ORDER_GET_ALL,
        web::get().to(get_orders_endpoint::<T>),
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{body::MessageBody, http::StatusCode, test::TestRequest, web::Data};
    use common::common_rest::{GenericErrorResponse, bad_request_type_url};
    use domain::test_fixtures::*;
    use dotenvy::dotenv;

    use super::*;
    use crate::test_fixtures::{MockGetOrders, rnd_order_details};

    #[actix_web::test]
    async fn limit_reached() {
        dotenv().ok();
        let start_id = rnd_order_id();
        let limit = 10;

        let mock_get_orders = AM::new_am(MockGetOrders {
            response: Err(GetOrdersUseCaseError::new_limit_exceed(limit + 1)),
            start_id,
            limit,
        });

        let mock_shared_state = Data::new(mock_get_orders.clone());
        let req = TestRequest::default()
            .uri(&format!("/?startId={}&limit={}", start_id.to_i64(), limit))
            .to_http_request();

        dbg!(&req);

        let resp = get_orders_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: GenericErrorResponse = serde_json::from_str(body_text).unwrap();

        assert_eq!(&response_dto.response_type, &bad_request_type_url());
        assert_eq!(
            &response_dto.response_status,
            &StatusCode::BAD_REQUEST.as_u16()
        );
        assert_eq!(&response_dto.response_title, "Bad request");
        assert_eq!(
            &response_dto.invalid_params.first().unwrap().message,
            "Max limit is 10"
        );
        mock_get_orders
            .lock_un()
            .verify_invoked(&start_id, &(limit + 1));
    }

    #[actix_web::test]
    async fn returned_successfully_without_next_page() {
        dotenv().ok();
        let limit = 1;

        let single = rnd_order_details(Default::default());
        let first_item = single.clone().items[0];

        let mock_get_orders = AM::new_am(MockGetOrders {
            response: Ok(vec![single.clone()]),
            start_id: single.id,
            limit,
        });

        let mock_shared_state = Data::new(mock_get_orders.clone());
        let req = TestRequest::default()
            .uri(&format!("/?startId={}&limit={}", single.id.to_i64(), limit))
            .to_http_request();

        let resp = get_orders_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: CursorPagedModel<OrderModel, i32> =
            serde_json::from_str(body_text).unwrap();

        assert_eq!(response_dto.list.len(), limit);
        assert_eq!(response_dto.list[0].id, single.id.to_i64());
        assert_eq!(
            response_dto.list[0].total_price,
            single.total.to_string_value()
        );
        assert_eq!(response_dto.list[0].version, single.version.to_i64());
        assert_eq!(
            response_dto.list[0].address.street,
            single.address.street_to_string()
        );
        assert_eq!(
            response_dto.list[0].address.building,
            single.address.building_to_i16()
        );
        assert_eq!(response_dto.list[0].items.len(), 1);
        assert_eq!(
            response_dto.list[0].items[0].meal_id,
            first_item.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.list[0].items[0].count,
            first_item.count.to_i32()
        );
        mock_get_orders
            .lock_un()
            .verify_invoked(&single.id, &(limit + 1));
    }

    #[actix_web::test]
    async fn returned_successfully_with_next_page() {
        dotenv().ok();
        let limit = 1;

        let first = rnd_order_details(Default::default());
        let first_item = first.clone().items[0];
        let second = rnd_order_details(Default::default());

        let mock_get_orders = AM::new_am(MockGetOrders {
            response: Ok(vec![first.clone(), second]),
            start_id: first.id,
            limit,
        });

        let mock_shared_state = Data::new(mock_get_orders.clone());
        let req = TestRequest::default()
            .uri(&format!("/?startId={}&limit={}", first.id.to_i64(), limit))
            .to_http_request();

        let resp = get_orders_endpoint(mock_shared_state, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = resp.into_body().try_into_bytes().unwrap();
        let body_text = std::str::from_utf8(&body).unwrap();

        let response_dto: CursorPagedModel<OrderModel, i64> =
            serde_json::from_str(body_text).unwrap();

        dbg!(&response_dto);

        assert_eq!(response_dto.list.len(), limit);
        assert_eq!(response_dto.list[0].id, first.id.to_i64());
        assert_eq!(
            response_dto.list[0].total_price,
            first.total.to_string_value()
        );
        assert_eq!(response_dto.list[0].version, first.version.to_i64());
        assert_eq!(
            response_dto.list[0].address.street,
            first.address.street_to_string()
        );
        assert_eq!(
            response_dto.list[0].address.building,
            first.address.building_to_i16()
        );
        assert_eq!(response_dto.list[0].items.len(), 1);
        assert_eq!(
            response_dto.list[0].items[0].meal_id,
            first_item.meal_id.to_i64()
        );
        assert_eq!(
            response_dto.list[0].items[0].count,
            first_item.count.to_i32()
        );
        mock_get_orders
            .lock_un()
            .verify_invoked(&first.id, &(limit + 1));
    }
}
