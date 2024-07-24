use crate::mappers::employee_mapper::EmployeeMapper;
use crate::models::employee::{Employee, PatchEmployee, PostEmployee};
use crate::models::employee_filter::EmployeeFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct EmployeeService {}

impl ServiceCRUD for EmployeeService {
    type Item = Employee;
    type PostItem = PostEmployee;
    type PatchItem = PatchEmployee;
    type Filter = EmployeeFilter;
    type Mapper = EmployeeMapper;
}

#[cfg(test)]
mod test {
    use crate::services::employee_service::EmployeeService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_employee() {
        use crate::models::employee::PostEmployee;
        let employee = PostEmployee::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match EmployeeService::add_single(pool, &employee) {
            Ok(result) => println!("{result}"),
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
