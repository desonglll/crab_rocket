use crate::models::order::{Order, PatchOrder, PostOrder};
use crate::models::order_filter::OrderFilter;
use crate::services::order_service::OrderService;
use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use rocket::State;
use std::error::Error;

pub struct OrderController {}

impl ControllerCRUD for OrderController {
    type Item = Order;
    type PostItem = PostOrder;
    type PatchItem = PatchOrder;
    type Param = RequestParam<OrderFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<OrderFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, OrderService, OrderFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, OrderService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostOrder,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, OrderService, PostOrder>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, OrderService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchOrder,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, OrderService, PatchOrder>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<OrderFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, OrderService, OrderFilter>(pool, param)
    }
}
