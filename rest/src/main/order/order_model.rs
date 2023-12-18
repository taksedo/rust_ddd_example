use actix_web::{http::header::ContentType, HttpResponse};
use common::types::main::common::address::Address;
use derive_new::new;
use serde::Serialize;
use serde_derive::Deserialize;
use usecase::main::order::dto::order_details::{OrderDetails, OrderItemDetails};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderModel {
    pub id: i64,
    pub address: AddressModel,
    pub items: Vec<OrderItemModel>,
    pub total_price: String,
    pub version: i64,
    pub ready_for_confirm_or_cancel: bool,
}

#[derive(new, Serialize, Deserialize, Debug, Clone)]
pub struct OrderItemModel {
    pub meal_id: i64,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressModel {
    pub street: String,
    pub building: i16,
}

/// Add conversion from usecase to rest objects
pub trait ToModel<T>: Sized {
    fn to_model(self) -> T;
}

impl ToModel<AddressModel> for Address {
    fn to_model(self) -> AddressModel {
        AddressModel {
            street: self.street_to_string(),
            building: self.building_to_i16(),
        }
    }
}

impl ToModel<Vec<OrderItemModel>> for Vec<OrderItemDetails> {
    fn to_model(self) -> Vec<OrderItemModel> {
        self.iter()
            .map(|&it| OrderItemModel::new(it.meal_id.to_i64(), it.count.to_i32()))
            .collect()
    }
}

impl ToModel<OrderModel> for OrderDetails {
    fn to_model(self) -> OrderModel {
        OrderModel {
            id: self.id.to_i64(),
            address: self.address.to_model(),
            items: self.items.to_model(),
            total_price: self.total.to_string_value(),
            version: self.version.to_i64(),
            ready_for_confirm_or_cancel: self.ready_for_confirm_or_cancel,
        }
    }
}

impl ToModel<HttpResponse> for OrderDetails {
    fn to_model(self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&ToModel::<OrderModel>::to_model(self)).unwrap())
    }
}
