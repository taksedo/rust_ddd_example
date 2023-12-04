use domain::main::order::value_objects::shop_order_id::ShopOrderId;

pub trait CompleteOrder {
    fn execute(&self, order_id: ShopOrderId) -> Result<(), CompleteOrderUseCaseError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompleteOrderUseCaseError {
    OrderNotFound,
    InvalidOrderState,
}
