use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;
use crate::services::order_service::OrderService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct OrderController {}

impl ControllerCRUD for OrderController {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Param = RequestParam<OrderFilter>;
    fn get_all(
        param: &RequestParam<OrderFilter>,
    ) -> Result<ApiResponse<Data<Vec<Order>>>, Box<dyn Error>> {
        controller_get_all::<Order, OrderService, OrderFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Order>, Box<dyn Error>> {
        controller_get_by_id::<Order, OrderService>(pid)
    }

    fn add_single(obj: &mut PostOrder) -> Result<ApiResponse<Order>, Box<dyn Error>> {
        controller_add_single::<Order, OrderService, PostOrder>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Order>, Box<dyn Error>> {
        controller_delete_by_id::<Order, OrderService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchOrder) -> Result<ApiResponse<Order>, Box<dyn Error>> {
        controller_update_by_id::<Order, OrderService, PatchOrder>(pid, obj)
    }
    fn filter(
        param: &RequestParam<OrderFilter>,
    ) -> Result<ApiResponse<Data<Vec<Order>>>, Box<dyn std::error::Error>> {
        controller_filter::<Order, OrderService, OrderFilter>(param)
    }
}
