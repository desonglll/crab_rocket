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

use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;
use crate::services::inventory_service::InventoryService;

pub struct InventoryController {}

impl ControllerCRUD for InventoryController {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Param = RequestParam<InventoryFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<InventoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, InventoryService, InventoryFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, InventoryService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostInventory,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, InventoryService, PostInventory>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, InventoryService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchInventory,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, InventoryService, PatchInventory>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<InventoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, InventoryService, InventoryFilter>(pool, param)
    }
}
