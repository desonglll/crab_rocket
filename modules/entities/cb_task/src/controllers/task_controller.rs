use std::error::Error;

use rocket::State;

use auth::models::session::Session;
use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;

use crate::models::task::{PatchTask, PostTask, Task};
use crate::models::task_filter::TaskFilter;
use crate::services::task_service::TaskService;

pub struct TaskController {}

impl ControllerCRUD for TaskController {
    type Item = Task;
    type PostItem = PostTask;
    type PatchItem = PatchTask;
    type Param = RequestParam<TaskFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<TaskFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, TaskService, TaskFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, TaskService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostTask,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, TaskService, PostTask>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, TaskService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchTask,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, TaskService, PatchTask>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<TaskFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => controller_filter::<Self::Item, TaskService, TaskFilter>(pool, param),
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
}
