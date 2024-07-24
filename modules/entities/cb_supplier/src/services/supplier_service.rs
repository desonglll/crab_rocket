use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::supplier_mapper::SupplierMapper;
use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;

pub struct SupplierService {}

impl ServiceCRUD for SupplierService {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Param = RequestParam<SupplierFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<SupplierFilter>,
    ) -> Result<Data<Vec<Supplier>>, Box<dyn Error>> {
        service_get_all::<Supplier, SupplierMapper, SupplierFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Supplier, Box<dyn Error>> {
        service_get_by_id::<Supplier, SupplierMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostSupplier) -> Result<Supplier, Box<dyn Error>> {
        service_add_single::<Supplier, SupplierMapper, PostSupplier>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Supplier, Box<dyn Error>> {
        service_delete_by_id::<Supplier, SupplierMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchSupplier,
    ) -> Result<Supplier, Box<dyn Error>> {
        service_update_by_id::<Supplier, SupplierMapper, PatchSupplier>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<SupplierFilter>,
    ) -> Result<Data<Vec<Supplier>>, Box<dyn std::error::Error>> {
        service_filter::<Supplier, SupplierMapper, SupplierFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::supplier_service::SupplierService;

    #[test]
    fn test_insert_single_supplier() {
        use crate::models::supplier::PostSupplier;
        let supplier = PostSupplier::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match SupplierService::add_single(pool, &supplier) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_suppliers() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match SupplierService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_supplier_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match SupplierService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_supplier_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match SupplierService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
