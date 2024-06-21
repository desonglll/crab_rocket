use crate::{
    models::employee::{Employee, NewEmployee, PatchEmployee},
    routes::employee_param::EmployeeParam,
    services::employee_service::GetEmployee,
};

pub fn insert_single_employee_controller(new_employee: &NewEmployee) -> (i32, String, Employee) {
    match Employee::insert_employee(new_employee) {
        Ok(result) => (200, String::from("INSERT EMPLOYEE OK"), result),
        Err(e) => (204, e.to_string(), Employee::default()),
    }
}
pub fn get_employee_by_id_controller(id: i32) -> (i32, String, Employee) {
    match Employee::get_employee_by_id(id) {
        Ok(result) => (200, String::from("GET EMPLOYEE BY ID OK"), result),
        Err(e) => (204, e.to_string(), Employee::default()),
    }
}
pub fn delete_employee_by_id_controller(id: i32) -> (i32, String, Employee) {
    match Employee::delete_employee_by_id(id) {
        Ok(result) => (200, String::from("DELETE EMPLOYEE BY ID OK"), result),
        Err(e) => (204, e.to_string(), Employee::default()),
    }
}
pub fn get_employee_by_params_controller(params: &EmployeeParam) -> (i32, String, Vec<Employee>) {
    match Employee::filter_employee_by_params(params) {
        Ok(filtered_emp) => (200, String::from("GET EMPLOYEE BY PARAMS OK"), filtered_emp),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}
pub fn update_employee_by_id_controller(id: i32, emp: &PatchEmployee) -> (i32, String, Employee) {
    match Employee::update_employee_by_id(id, emp) {
        Ok(updated_emp) => (200, String::from("UPDATED EMPLOYEE OK"), updated_emp),
        Err(e) => (204, e.to_string(), Employee::default()),
    }
}
