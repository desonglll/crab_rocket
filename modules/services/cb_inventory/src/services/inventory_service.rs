use crate::mappers::inventory_mapper::InventoryMapper;
use crate::models::inventory::{Inventory, PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct InventoryService {}

impl ServiceCRUD for InventoryService {
    type Item = Inventory;
    type PostItem = PostInventory;
    type PatchItem = PatchInventory;
    type Filter = InventoryFilter;
    type Mapper = InventoryMapper;
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
            Ok(result) => println!("{result}"),
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
