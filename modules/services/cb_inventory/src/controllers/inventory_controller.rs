use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;
use crate::services::inventory_service::InventoryService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct InventoryController {}

impl ControllerCRUD for InventoryController {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Param = RequestParam<PaginationParam, InventoryFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, InventoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Inventory>>>, Box<dyn Error>> {
        controller_get_all::<Inventory, InventoryService, InventoryFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Inventory>, Box<dyn Error>> {
        controller_get_by_id::<Inventory, InventoryService>(pid)
    }

    fn add_single(obj: &mut PostInventory) -> Result<ApiResponse<Inventory>, Box<dyn Error>> {
        controller_add_single::<Inventory, InventoryService, PostInventory>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Inventory>, Box<dyn Error>> {
        controller_delete_by_id::<Inventory, InventoryService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchInventory,
    ) -> Result<ApiResponse<Inventory>, Box<dyn Error>> {
        controller_update_by_id::<Inventory, InventoryService, PatchInventory>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, InventoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Inventory>>>, Box<dyn std::error::Error>> {
        controller_filter::<Inventory, InventoryService, InventoryFilter>(param)
    }
}
