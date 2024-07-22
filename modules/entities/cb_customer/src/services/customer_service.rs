use crate::mappers::customer_mapper::CustomerMapper;
use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

pub struct CustomerService {}

impl ServiceCRUD for CustomerService {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Param = RequestParam<CustomerFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<CustomerFilter>,
    ) -> Result<Data<Vec<Customer>>, Box<dyn Error>> {
        service_get_all::<Customer, CustomerMapper, CustomerFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Customer, Box<dyn Error>> {
        service_get_by_id::<Customer, CustomerMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostCustomer) -> Result<Customer, Box<dyn Error>> {
        service_add_single::<Customer, CustomerMapper, PostCustomer>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Customer, Box<dyn Error>> {
        service_delete_by_id::<Customer, CustomerMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchCustomer,
    ) -> Result<Customer, Box<dyn Error>> {
        service_update_by_id::<Customer, CustomerMapper, PatchCustomer>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<CustomerFilter>,
    ) -> Result<Data<Vec<Customer>>, Box<dyn std::error::Error>> {
        service_filter::<Customer, CustomerMapper, CustomerFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::customer_service::CustomerService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_customer() {
        use crate::models::customer::PostCustomer;
        let customer = PostCustomer::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerService::add_single(pool, &customer) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_customers() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_customer_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_customer_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CustomerService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
