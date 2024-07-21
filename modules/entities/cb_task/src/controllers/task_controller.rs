use crate::models::task::{PostTask, PatchTask, Task};
use crate::models::task_filter::TaskFilter;
use crate::services::task_service::TaskService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct TaskController {}

impl ControllerCRUD for TaskController {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<PaginationParam, TaskFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, TaskService, TaskFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, TaskService>(pid)
    }

    fn add_single(obj: &mut PostTask) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, TaskService, PostTask>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, TaskService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchTask) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, TaskService, PatchTask>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, TaskFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, TaskService, TaskFilter>(param)
    }
}
