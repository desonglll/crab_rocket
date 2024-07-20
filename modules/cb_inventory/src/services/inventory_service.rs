use crate::mappers::inventory_mapper::InventoryMapper;
use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct InventoryService {}

impl ServiceCRUD for InventoryService {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Param = RequestParam<PaginationParam, InventoryFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, InventoryFilter>,
    ) -> Result<Data<Vec<Inventory>>, Box<dyn Error>> {
        service_get_all::<Inventory, InventoryMapper, InventoryFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Inventory, Box<dyn Error>> {
        service_get_by_id::<Inventory, InventoryMapper>(pid)
    }

    fn add_single(obj: &PostInventory) -> Result<Inventory, Box<dyn Error>> {
        service_add_single::<Inventory, InventoryMapper, PostInventory>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Inventory, Box<dyn Error>> {
        service_delete_by_id::<Inventory, InventoryMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchInventory) -> Result<Inventory, Box<dyn Error>> {
        service_update_by_id::<Inventory, InventoryMapper, PatchInventory>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, InventoryFilter>,
    ) -> Result<Data<Vec<Inventory>>, Box<dyn std::error::Error>> {
        service_filter::<Inventory, InventoryMapper, InventoryFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::inventory_service::InventoryService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_inventory() {
        use crate::models::inventory::PostInventory;
        let inventory = PostInventory::default();
        match InventoryService::add_single(&inventory) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_inventorys() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match InventoryService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_inventory_by_id() {
        match InventoryService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_inventory_by_id() {
        match InventoryService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
