use crate::mappers::supplier_mapper::SupplierMapper;
use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;
use obj_traits::service::service_crud::ServiceCRUD;

pub struct SupplierService {}

impl ServiceCRUD for SupplierService {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Filter = SupplierFilter;
    type Mapper = SupplierMapper;
}

#[cfg(test)]
mod test {
    use crate::services::supplier_service::SupplierService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_supplier() {
        use crate::models::supplier::PostSupplier;
        let supplier = PostSupplier::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match SupplierService::add_single(pool, &supplier) {
            Ok(result) => println!("{result}"),
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
