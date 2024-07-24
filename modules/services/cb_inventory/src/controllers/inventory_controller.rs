use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;
use crate::services::inventory_service::InventoryService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct InventoryController {}

impl ControllerCRUD for InventoryController {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Filter = InventoryFilter;
    type Service = InventoryService;
}
