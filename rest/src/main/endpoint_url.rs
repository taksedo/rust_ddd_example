pub const API_V1: &str = "/rest/shop/v1";

pub const MENU: &str = "/menu";
pub const MENU_ADD_TO_MENU: &str = "/add";
pub const MENU_GET_BY_ID: &str = "/{id}";
pub const MENU_GET_ALL: &str = "/";
pub const MENU_DELETE_BY_ID: &str = "/{id}";

pub const ORDER: &str = "$API_V1/orders";
pub const ORDER_CANCEL_BY_ID: &str = "/{id}/cancel";
pub const ORDER_CONFIRM_BY_ID: &str = "/{id}/confirm";
pub const ORDER_GET_BY_ID: &str = "/{id}";
pub const ORDER_GET_ALL: &str = "/";
pub const ORDER_GET_WITH_PAGINATION: &str = "/?startId={startId}&limit={limit}";
