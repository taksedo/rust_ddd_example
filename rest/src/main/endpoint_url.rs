use const_format::*;

pub const API_V1: &str = "/rest/shop/v1";

pub const API_V1_MENU: &str = formatcp!("{API_V1}/menu");
pub const API_V1_MENU_ADD_TO_MENU: &str = formatcp!("{API_V1_MENU}/add");
pub const API_V1_MENU_GET_BY_ID: &str = formatcp!("{API_V1_MENU}/{{id}}");
pub const API_V1_MENU_GET_ALL: &str = formatcp!("{API_V1_MENU}");
pub const API_V1_MENU_DELETE_BY_ID: &str = formatcp!("{API_V1_MENU}/{{id}}");

pub const API_V1_ORDER: &str = formatcp!("{API_V1}/orders");
pub const API_V1_ORDER_CANCEL_BY_ID: &str = formatcp!("{API_V1}/{{id}}/cancel");
pub const API_V1_ORDER_CONFIRM_BY_ID: &str = formatcp!("{API_V1}/{{id}}/confirm");
pub const API_V1_ORDER_GET_BY_ID: &str = formatcp!("{API_V1}/{{id}}");
pub const API_V1_ORDER_GET_ALL: &str = formatcp!("{API_V1}/");
pub const API_V1_ORDER_GET_WITH_PAGINATION: &str =
    formatcp!("{API_V1}/?startId={{startId}}&limit={{limit}}");
