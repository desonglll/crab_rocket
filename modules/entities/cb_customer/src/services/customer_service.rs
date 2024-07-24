use crate::mappers::customer_mapper::CustomerMapper;
use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct CustomerService {}

impl ServiceCRUD for CustomerService {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Filter = CustomerFilter;
    type Mapper = CustomerMapper;
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
            Ok(result) => println!("{result}"),
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
