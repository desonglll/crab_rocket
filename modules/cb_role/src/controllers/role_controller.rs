use crate::models::role::{NewRole, PatchRole, Role};
use crate::services::role_service::RoleService;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use std::error::Error;

pub struct RoleController {}

impl ControllerCRUD<Role, NewRole, PatchRole, RequestParam<PaginationParam>> for RoleController {
    fn get_all(
        param: &RequestParam<PaginationParam>,
    ) -> Result<ApiResponse<Data<Vec<Role>>>, Box<dyn Error>> {
        match RoleService::get_all(param) {
            Ok(all_roles) => Ok(ApiResponse::success(all_roles)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Role>, Box<dyn Error>> {
        match RoleService::get_by_id(pid) {
            Ok(role) => Ok(ApiResponse::success(role)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn add_single(obj: &mut NewRole) -> Result<ApiResponse<Role>, Box<dyn Error>> {
        match RoleService::add_single(obj) {
            Ok(result_task) => Ok(ApiResponse::success(result_task)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Role>, Box<dyn Error>> {
        match RoleService::delete_by_id(pid) {
            Ok(deleted_role) => Ok(ApiResponse::success(deleted_role)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn update_by_id(pid: i32, obj: &PatchRole) -> Result<ApiResponse<Role>, Box<dyn Error>> {
        match RoleService::update_by_id(pid, obj) {
            Ok(updated_role) => Ok(ApiResponse::success(updated_role)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
