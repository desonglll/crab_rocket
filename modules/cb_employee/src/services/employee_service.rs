use crate::mappers::employee_mapper::EmployeeMapper;
use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::models::employee_filter::EmployeeFilter;
use crab_rocket_schema::establish_pg_connection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use std::error::Error;

pub struct EmployeeService {}

impl
    ServiceCRUD<
        Employee,
        NewEmployee,
        PatchEmployee,
        RequestParam<PaginationParam, EmployeeFilter>,
        EmployeeFilter,
    > for EmployeeService
{
    fn get_all(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::get_all(&mut conn, param) {
                Ok(all_employees) => Ok(all_employees),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn get_by_id(pid: i32) -> Result<Employee, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::get_by_id(&mut conn, pid) {
                Ok(employee) => Ok(employee),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn add_single(obj: &NewEmployee) -> Result<Employee, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::add_single(&mut conn, obj) {
                Ok(employee) => Ok(employee),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<Employee, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::delete_by_id(&mut conn, pid) {
                Ok(deleted_employee) => Ok(deleted_employee),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn update_by_id(pid: i32, obj: &PatchEmployee) -> Result<Employee, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::update_by_id(&mut conn, pid, obj) {
                Ok(updated_employee) => Ok(updated_employee),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn filter(param: &EmployeeFilter) -> Result<Data<Vec<Employee>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::filter(&mut conn, param) {
                Ok(all_employees) => Ok(all_employees),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::services::employee_service::EmployeeService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_employee() {
        use crate::models::employee::NewEmployee;
        let employee = NewEmployee::demo();
        match EmployeeService::add_single(&employee) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_employees() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match EmployeeService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_employee_by_id() {
        match EmployeeService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_employee_by_id() {
        match EmployeeService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
