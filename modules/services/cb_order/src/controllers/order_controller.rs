use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;
use crate::services::order_service::OrderService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct OrderController {}

impl ControllerCRUD for OrderController {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Filter = OrderFilter;
    type Service = OrderService;
}
