use std::error::Error;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationRequestParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use crate::models::task::{NewTask, PatchTask, Task};
use crate::services::task_service::{TaskService};

pub struct TaskController {}

impl ControllerCRUD<Task, NewTask, PatchTask, RequestParam<PaginationRequestParam>> for TaskController {
    fn get_all(param: &RequestParam<PaginationRequestParam>) -> Result<ApiResponse<Data<Vec<Task>>>, Box<dyn Error>> {
        match TaskService::get_all(param) {
            Ok(data) => {
                let response =
                    ApiResponse::new("200".to_string(), "Success".to_string(), data);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::new("200".to_string(), e.to_string(), Data::new(Vec::new(), Pagination::default())))
            }
        }
    }
    fn get_by_id(pid: i32) -> Result<ApiResponse<Task>, Box<dyn Error>> {
        match TaskService::get_by_id(pid) {
            Ok(body) => {
                let response = ApiResponse::new(String::from("200"), String::from("Success"), body);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::new("200".to_string(), e.to_string(), Task::default()))
            }
        }
    }

    fn add_single(obj: &mut NewTask) -> Result<ApiResponse<Task>, Box<dyn Error>> {
        // Handle None date time
        if obj.created_at() == None {
            obj.set_created_at(Some(get_e8_time()));
        }
        if obj.updated_at() == None {
            obj.set_updated_at(Some(get_e8_time()));
        }
        match TaskService::add_single(&obj) {
            Ok(result_task) => {
                let response = ApiResponse::success(result_task);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error())
            }
        }
    }
    fn delete_by_id(pid: i32) -> Result<ApiResponse<Task>, Box<dyn Error>> {
        match TaskService::delete_by_id(pid) {
            Ok(deleted_task) => {
                let response = ApiResponse::success(deleted_task);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error())
            }
        }
    }
    fn update_by_id(pid: i32, obj: &PatchTask) -> Result<ApiResponse<Task>, Box<dyn Error>> {
        match TaskService::update_by_id(pid, &obj) {
            Ok(patched_task) => {
                let response = ApiResponse::success(patched_task);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error())
            }
        }
    }
}

