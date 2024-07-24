use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;

use crate::models::employee::{Employee, PatchEmployee, PostEmployee};
use crate::models::employee_filter::EmployeeFilter;
use crate::services::employee_service::EmployeeService;

pub struct EmployeeController {}

impl ControllerCRUD for EmployeeController {
    type Item = Employee;
    type PostItem = PostEmployee;
    type PatchItem = PatchEmployee;
    type Param = RequestParam<EmployeeFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, EmployeeService, EmployeeFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, EmployeeService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostEmployee,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, EmployeeService, PostEmployee>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, EmployeeService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchEmployee,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, EmployeeService, PatchEmployee>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<EmployeeFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, EmployeeService, EmployeeFilter>(pool, param)
    }
}
