use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;
use crate::services::customer_service::CustomerService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct CustomerController {}

impl ControllerCRUD for CustomerController {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Filter = CustomerFilter;
    type Service = CustomerService;
}
