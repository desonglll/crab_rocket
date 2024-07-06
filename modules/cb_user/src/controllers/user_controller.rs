use crate::models::user::{NewUser, PatchUser, User};
use crate::models::user_filter::UserFilter;
use crate::services::user_service::UserService;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use std::error::Error;

pub struct UserController {}

impl ControllerCRUD<User, NewUser, PatchUser, RequestParam<PaginationParam, UserFilter>>
    for UserController
{
    fn get_all(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<ApiResponse<Data<Vec<User>>>, Box<dyn Error>> {
        match UserService::get_all(param) {
            Ok(all_users) => {
                let response = ApiResponse::success(all_users);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<User>, Box<dyn Error>> {
        match UserService::get_by_id(pid) {
            Ok(user) => {
                let response = ApiResponse::success(user);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn add_single(obj: &mut NewUser) -> Result<ApiResponse<User>, Box<dyn Error>> {
        match UserService::add_single(obj) {
            Ok(user) => {
                let response = ApiResponse::success(user);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<User>, Box<dyn Error>> {
        match UserService::delete_by_id(pid) {
            Ok(user) => {
                let response = ApiResponse::success(user);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn update_by_id(pid: i32, obj: &PatchUser) -> Result<ApiResponse<User>, Box<dyn Error>> {
        match UserService::update_by_id(pid, obj) {
            Ok(user) => {
                let response = ApiResponse::success(user);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
    fn filter(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<ApiResponse<Data<Vec<User>>>, Box<dyn std::error::Error>> {
        match UserService::filter(param) {
            Ok(all_users) => {
                let response = ApiResponse::success(all_users);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
