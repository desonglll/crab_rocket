use crate::{
    models::{
        employee::{Employee, NewEmployee, PatchEmployee},
        info::employee_info::EmployeeInfo,
    },
    routes::models::employee_param::EmployeeParam,
    services::employee_service::GetEmployee,
};

pub fn insert_single_employee_controller(
    new_employee: &NewEmployee,
) -> (i32, &'static str, Employee) {
    match Employee::insert_employee(new_employee) {
        Ok(result) => (200, "INSERT EMPLOYEE OK", result),
        Err(_) => (204, "INSERT EMPLOYEE ERROR", Employee::new_empty("-1")),
    }
}

pub fn delete_employee_by_id_controller(id: i32) -> (i32, &'static str, Employee) {
    match Employee::delete_employee_by_id(id) {
        Ok(result) => (200, "DELETE EMPLOYEE BY ID OK", result),
        Err(_) => (
            204,
            "DELETE EMPLOYEE BY ID ERROR",
            Employee::new_empty("-1"),
        ),
    }
}
pub fn get_employee_by_params_controller(
    params: &EmployeeParam,
) -> (i32, &'static str, (Vec<Employee>, EmployeeInfo)) {
    match Employee::filter_employee_by_params(params).0 {
        Ok(filtered_emp) => (
            200,
            "GET EMPLOYEE BY PARAMS OK",
            (filtered_emp, Employee::filter_employee_by_params(params).1),
        ),
        Err(_) => (
            204,
            "GET EMPLOYEE BY PARAMS ERROR",
            (Vec::new(), EmployeeInfo::new_empty()),
        ),
    }
}
pub fn update_employee_by_id_controller(
    id: i32,
    emp: &PatchEmployee,
) -> (i32, &'static str, Employee) {
    match Employee::update_employee_by_id(id, emp) {
        Ok(updated_emp) => (200, "UPDATED EMPLOYEE OK", updated_emp),
        Err(_) => (204, "UPDATED EMPLOYEE ERROR", Employee::new_empty("-1")),
    }
}
