use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;
use crate::services::supplier_service::SupplierService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct SupplierController {}

impl ControllerCRUD for SupplierController {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Param = RequestParam<SupplierFilter>;
    fn get_all(
        param: &RequestParam<SupplierFilter>,
    ) -> Result<ApiResponse<Data<Vec<Supplier>>>, Box<dyn Error>> {
        controller_get_all::<Supplier, SupplierService, SupplierFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Supplier>, Box<dyn Error>> {
        controller_get_by_id::<Supplier, SupplierService>(pid)
    }

    fn add_single(obj: &mut PostSupplier) -> Result<ApiResponse<Supplier>, Box<dyn Error>> {
        controller_add_single::<Supplier, SupplierService, PostSupplier>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Supplier>, Box<dyn Error>> {
        controller_delete_by_id::<Supplier, SupplierService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchSupplier,
    ) -> Result<ApiResponse<Supplier>, Box<dyn Error>> {
        controller_update_by_id::<Supplier, SupplierService, PatchSupplier>(pid, obj)
    }
    fn filter(
        param: &RequestParam<SupplierFilter>,
    ) -> Result<ApiResponse<Data<Vec<Supplier>>>, Box<dyn std::error::Error>> {
        controller_filter::<Supplier, SupplierService, SupplierFilter>(param)
    }
}
