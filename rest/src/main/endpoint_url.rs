use const_format::*;

pub const API_V1: &str = "/rest/shop/v1";

pub const MENU: &str = formatcp!("{API_V1}/menu");
pub const MENU_ADD_TO_MENU: &str = formatcp!("{MENU}/add");
pub const MENU_GET_BY_ID: &str = formatcp!("{MENU}/{{id}}");
pub const MENU_GET_ALL: &str = formatcp!("{MENU}/");
pub const MENU_DELETE_BY_ID: &str = formatcp!("{MENU}/{{id}}");

pub const ORDER: &str = formatcp!("{API_V1}/orders");
pub const ORDER_CANCEL_BY_ID: &str = &formatcp!("{API_V1}/{{id}}/cancel");
pub const ORDER_CONFIRM_BY_ID: &str = &formatcp!("{API_V1}/{{id}}/confirm");
pub const ORDER_GET_BY_ID: &str = &formatcp!("{API_V1}/{{id}}");
pub const ORDER_GET_ALL: &str = &formatcp!("{API_V1}/");
pub const ORDER_GET_WITH_PAGINATION: &str =
    &formatcp!("{API_V1}/?startId={{startId}}&limit={{limit}}");
