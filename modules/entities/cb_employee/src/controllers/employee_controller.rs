use crate::models::employee::{Employee, PatchEmployee, PostEmployee};
use crate::models::employee_filter::EmployeeFilter;
use crate::services::employee_service::EmployeeService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct EmployeeController {}

impl ControllerCRUD for EmployeeController {
    type Item = Employee;
    type PostItem = PostEmployee;
    type PatchItem = PatchEmployee;
    type Filter = EmployeeFilter;
    type Service = EmployeeService;
}
