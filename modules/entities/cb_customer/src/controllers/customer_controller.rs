use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;
use crate::services::customer_service::CustomerService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct CustomerController {}

impl ControllerCRUD for CustomerController {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Param = RequestParam<CustomerFilter>;
    fn get_all(
        param: &RequestParam<CustomerFilter>,
    ) -> Result<ApiResponse<Data<Vec<Customer>>>, Box<dyn Error>> {
        controller_get_all::<Customer, CustomerService, CustomerFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Customer>, Box<dyn Error>> {
        controller_get_by_id::<Customer, CustomerService>(pid)
    }

    fn add_single(obj: &mut PostCustomer) -> Result<ApiResponse<Customer>, Box<dyn Error>> {
        controller_add_single::<Customer, CustomerService, PostCustomer>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Customer>, Box<dyn Error>> {
        controller_delete_by_id::<Customer, CustomerService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchCustomer,
    ) -> Result<ApiResponse<Customer>, Box<dyn Error>> {
        controller_update_by_id::<Customer, CustomerService, PatchCustomer>(pid, obj)
    }
    fn filter(
        param: &RequestParam<CustomerFilter>,
    ) -> Result<ApiResponse<Data<Vec<Customer>>>, Box<dyn std::error::Error>> {
        controller_filter::<Customer, CustomerService, CustomerFilter>(param)
    }
}
