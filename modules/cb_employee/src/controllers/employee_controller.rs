use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::models::employee_filter::EmployeeFilter;
use crate::services::employee_service::EmployeeService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct EmployeeController {}

impl ControllerCRUD for EmployeeController {
    type Item = Employee;
    type NewItem = NewEmployee;
    type PatchItem = PatchEmployee;
    type Param = RequestParam<PaginationParam, EmployeeFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Employee>>>, Box<dyn Error>> {
        controller_get_all::<Employee, EmployeeService, EmployeeFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        controller_get_by_id::<Employee, EmployeeService>(pid)
    }

    fn add_single(obj: &mut NewEmployee) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        controller_add_single::<Employee, EmployeeService, NewEmployee>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        controller_delete_by_id::<Employee, EmployeeService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchEmployee,
    ) -> Result<ApiResponse<Employee>, Box<dyn Error>> {
        controller_update_by_id::<Employee, EmployeeService, PatchEmployee>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Employee>>>, Box<dyn std::error::Error>> {
        controller_filter::<Employee, EmployeeService, EmployeeFilter>(param)
    }
}
