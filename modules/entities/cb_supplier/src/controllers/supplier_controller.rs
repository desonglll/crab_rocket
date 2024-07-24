use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;

use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;
use crate::services::supplier_service::SupplierService;

pub struct SupplierController {}

impl ControllerCRUD for SupplierController {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Param = RequestParam<SupplierFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<SupplierFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, SupplierService, SupplierFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, SupplierService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostSupplier,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, SupplierService, PostSupplier>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, SupplierService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchSupplier,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, SupplierService, PatchSupplier>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<SupplierFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, SupplierService, SupplierFilter>(pool, param)
    }
}
