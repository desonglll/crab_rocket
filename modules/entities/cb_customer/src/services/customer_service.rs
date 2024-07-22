use crate::mappers::customer_mapper::CustomerMapper;
use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct CustomerService {}

impl ServiceCRUD for CustomerService {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Param = RequestParam<CustomerFilter>;
    fn get_all(
        param: &RequestParam<CustomerFilter>,
    ) -> Result<Data<Vec<Customer>>, Box<dyn Error>> {
        service_get_all::<Customer, CustomerMapper, CustomerFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Customer, Box<dyn Error>> {
        service_get_by_id::<Customer, CustomerMapper>(pid)
    }

    fn add_single(obj: &PostCustomer) -> Result<Customer, Box<dyn Error>> {
        service_add_single::<Customer, CustomerMapper, PostCustomer>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Customer, Box<dyn Error>> {
        service_delete_by_id::<Customer, CustomerMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchCustomer) -> Result<Customer, Box<dyn Error>> {
        service_update_by_id::<Customer, CustomerMapper, PatchCustomer>(pid, obj)
    }
    fn filter(
        param: &RequestParam<CustomerFilter>,
    ) -> Result<Data<Vec<Customer>>, Box<dyn std::error::Error>> {
        service_filter::<Customer, CustomerMapper, CustomerFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::customer_service::CustomerService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_customer() {
        use crate::models::customer::PostCustomer;
        let customer = PostCustomer::default();
        match CustomerService::add_single(&customer) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_customers() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        match CustomerService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_customer_by_id() {
        match CustomerService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_customer_by_id() {
        match CustomerService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
