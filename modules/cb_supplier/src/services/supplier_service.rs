use crate::mappers::supplier_mapper::SupplierMapper;
use crate::models::supplier::{PatchSupplier, PostSupplier, Supplier};
use crate::models::supplier_filter::SupplierFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct SupplierService {}

impl ServiceCRUD for SupplierService {
    type Item = Supplier;
    type PostItem = PostSupplier;
    type PatchItem = PatchSupplier;
    type Param = RequestParam<PaginationParam, SupplierFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, SupplierFilter>,
    ) -> Result<Data<Vec<Supplier>>, Box<dyn Error>> {
        service_get_all::<Supplier, SupplierMapper, SupplierFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Supplier, Box<dyn Error>> {
        service_get_by_id::<Supplier, SupplierMapper>(pid)
    }

    fn add_single(obj: &PostSupplier) -> Result<Supplier, Box<dyn Error>> {
        service_add_single::<Supplier, SupplierMapper, PostSupplier>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Supplier, Box<dyn Error>> {
        service_delete_by_id::<Supplier, SupplierMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchSupplier) -> Result<Supplier, Box<dyn Error>> {
        service_update_by_id::<Supplier, SupplierMapper, PatchSupplier>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, SupplierFilter>,
    ) -> Result<Data<Vec<Supplier>>, Box<dyn std::error::Error>> {
        service_filter::<Supplier, SupplierMapper, SupplierFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::supplier_service::SupplierService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_supplier() {
        use crate::models::supplier::PostSupplier;
        let supplier = PostSupplier::demo();
        match SupplierService::add_single(&supplier) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_suppliers() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match SupplierService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_supplier_by_id() {
        match SupplierService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_supplier_by_id() {
        match SupplierService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
