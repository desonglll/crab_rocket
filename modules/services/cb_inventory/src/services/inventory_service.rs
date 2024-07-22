use crate::mappers::inventory_mapper::InventoryMapper;
use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

pub struct InventoryService {}

impl ServiceCRUD for InventoryService {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Param = RequestParam<InventoryFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<InventoryFilter>,
    ) -> Result<Data<Vec<Inventory>>, Box<dyn Error>> {
        service_get_all::<Inventory, InventoryMapper, InventoryFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Inventory, Box<dyn Error>> {
        service_get_by_id::<Inventory, InventoryMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostInventory) -> Result<Inventory, Box<dyn Error>> {
        service_add_single::<Inventory, InventoryMapper, PostInventory>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Inventory, Box<dyn Error>> {
        service_delete_by_id::<Inventory, InventoryMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchInventory,
    ) -> Result<Inventory, Box<dyn Error>> {
        service_update_by_id::<Inventory, InventoryMapper, PatchInventory>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<InventoryFilter>,
    ) -> Result<Data<Vec<Inventory>>, Box<dyn std::error::Error>> {
        service_filter::<Inventory, InventoryMapper, InventoryFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::inventory_service::InventoryService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_inventory() {
        use crate::models::inventory::PostInventory;
        let inventory = PostInventory::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match InventoryService::add_single(pool, &inventory) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_inventorys() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match InventoryService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_inventory_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match InventoryService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_inventory_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match InventoryService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
