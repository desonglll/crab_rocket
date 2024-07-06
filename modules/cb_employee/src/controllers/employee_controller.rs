use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::models::employee_filter::EmployeeFilter;
use crate::services::employee_service::EmployeeService;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use std::error::Error;

pub struct EmployeeController {}

impl
    ControllerCRUD<
        Employee,
        NewEmployee,
        PatchEmployee,
        RequestParam<PaginationParam, EmployeeFilter>,
    > for EmployeeController
{
    fn get_all(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Employee>>>, Box<dyn Error>> {
        match EmployeeService::get_all(param) {
            Ok(all_employees) => {
                let response = ApiResponse::success(all_employees);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        match EmployeeService::get_by_id(pid) {
            Ok(employee) => {
                let response = ApiResponse::success(employee);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn add_single(obj: &mut NewEmployee) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        match EmployeeService::add_single(obj) {
            Ok(employee) => {
                let response = ApiResponse::success(employee);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        match EmployeeService::delete_by_id(pid) {
            Ok(employee) => {
                let response = ApiResponse::success(employee);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchEmployee,
    ) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        match EmployeeService::update_by_id(pid, obj) {
            Ok(employee) => {
                let response = ApiResponse::success(employee);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
    fn filter(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Employee>>>, Box<dyn std::error::Error>> {
        match EmployeeService::filter(param) {
            Ok(all_employees) => {
                let response = ApiResponse::success(all_employees);
                Ok(response)
            }
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
