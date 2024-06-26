use crate::{
    models::employee::{Employee, NewEmployee, PatchEmployee},
    routes::employee_param::EmployeeParam,
};

pub trait GetEmployee {
    fn insert_employee(new_employee: &NewEmployee) -> Result<Employee, Box<dyn std::error::Error>>;
    fn delete_employee_by_id(id: i32) -> Result<Employee, Box<dyn std::error::Error>>;
    fn get_employee_by_id(id: i32) -> Result<Employee, Box<dyn std::error::Error>>;
    fn get_all_employees() -> Result<Vec<Employee>, Box<dyn std::error::Error>>;
    fn filter_employee_by_params(
        params: &EmployeeParam,
    ) -> Result<Vec<Employee>, Box<dyn std::error::Error>>;
    fn update_employee_by_id(
        id: i32,
        emp: &PatchEmployee,
    ) -> Result<Employee, Box<dyn std::error::Error>>;
}
