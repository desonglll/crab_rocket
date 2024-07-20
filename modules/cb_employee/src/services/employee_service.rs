use crate::mappers::employee_mapper::EmployeeMapper;
use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::models::employee_filter::EmployeeFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct EmployeeService {}

impl ServiceCRUD for EmployeeService {
    type Item = Employee;
    type NewItem = NewEmployee;
    type PatchItem = PatchEmployee;
    type Param = RequestParam<PaginationParam, EmployeeFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, Box<dyn Error>> {
        service_get_all::<Employee, EmployeeMapper, EmployeeFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Employee, Box<dyn Error>> {
        service_get_by_id::<Employee, EmployeeMapper>(pid)
    }

    fn add_single(obj: &NewEmployee) -> Result<Employee, Box<dyn Error>> {
        service_add_single::<Employee, EmployeeMapper, NewEmployee>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Employee, Box<dyn Error>> {
        service_delete_by_id::<Employee, EmployeeMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchEmployee) -> Result<Employee, Box<dyn Error>> {
        service_update_by_id::<Employee, EmployeeMapper, PatchEmployee>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, Box<dyn std::error::Error>> {
        service_filter::<Employee, EmployeeMapper, EmployeeFilter>(param)
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
