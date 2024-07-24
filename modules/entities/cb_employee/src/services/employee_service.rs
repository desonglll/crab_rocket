use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::employee_mapper::EmployeeMapper;
use crate::models::employee::{Employee, PatchEmployee, PostEmployee};
use crate::models::employee_filter::EmployeeFilter;

pub struct EmployeeService {}

impl ServiceCRUD for EmployeeService {
    type Item = Employee;
    type PostItem = PostEmployee;
    type PatchItem = PatchEmployee;
    type Param = RequestParam<EmployeeFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, Box<dyn Error>> {
        service_get_all::<Employee, EmployeeMapper, EmployeeFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Employee, Box<dyn Error>> {
        service_get_by_id::<Employee, EmployeeMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostEmployee) -> Result<Employee, Box<dyn Error>> {
        service_add_single::<Employee, EmployeeMapper, PostEmployee>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Employee, Box<dyn Error>> {
        service_delete_by_id::<Employee, EmployeeMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchEmployee,
    ) -> Result<Employee, Box<dyn Error>> {
        service_update_by_id::<Employee, EmployeeMapper, PatchEmployee>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, Box<dyn std::error::Error>> {
        service_filter::<Employee, EmployeeMapper, EmployeeFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::employee_service::EmployeeService;

    #[test]
    fn test_insert_single_employee() {
        use crate::models::employee::PostEmployee;
        let employee = PostEmployee::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match EmployeeService::add_single(pool, &employee) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_employees() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match EmployeeService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_employee_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match EmployeeService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_employee_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match EmployeeService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
