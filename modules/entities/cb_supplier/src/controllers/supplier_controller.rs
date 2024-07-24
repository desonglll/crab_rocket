use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;
use crate::services::supplier_service::SupplierService;

pub struct SupplierController {}

impl ControllerCRUD for SupplierController {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Service = SupplierService;
    type Filter = SupplierFilter;
}
