use crate::models::customer::{Customer, PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;
use crate::services::customer_service::CustomerService;
use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use rocket::State;
use std::error::Error;

pub struct CustomerController {}

impl ControllerCRUD for CustomerController {
    type Item = Customer;
    type PostItem = PostCustomer;
    type PatchItem = PatchCustomer;
    type Param = RequestParam<CustomerFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<CustomerFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, CustomerService, CustomerFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, CustomerService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostCustomer,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, CustomerService, PostCustomer>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, CustomerService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchCustomer,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, CustomerService, PatchCustomer>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<CustomerFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, CustomerService, CustomerFilter>(pool, param)
    }
}
