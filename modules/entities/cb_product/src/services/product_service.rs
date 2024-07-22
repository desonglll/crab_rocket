use crate::mappers::product_mapper::ProductMapper;
use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct ProductService {}

impl ServiceCRUD for ProductService {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Param = RequestParam<ProductFilter>;
    fn get_all(param: &RequestParam<ProductFilter>) -> Result<Data<Vec<Product>>, Box<dyn Error>> {
        service_get_all::<Product, ProductMapper, ProductFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Product, Box<dyn Error>> {
        service_get_by_id::<Product, ProductMapper>(pid)
    }

    fn add_single(obj: &PostProduct) -> Result<Product, Box<dyn Error>> {
        service_add_single::<Product, ProductMapper, PostProduct>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Product, Box<dyn Error>> {
        service_delete_by_id::<Product, ProductMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchProduct) -> Result<Product, Box<dyn Error>> {
        service_update_by_id::<Product, ProductMapper, PatchProduct>(pid, obj)
    }
    fn filter(
        param: &RequestParam<ProductFilter>,
    ) -> Result<Data<Vec<Product>>, Box<dyn std::error::Error>> {
        service_filter::<Product, ProductMapper, ProductFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::product_service::ProductService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_product() {
        use crate::models::product::PostProduct;
        let product = PostProduct::default();
        match ProductService::add_single(&product) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_products() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        match ProductService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_product_by_id() {
        match ProductService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_product_by_id() {
        match ProductService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
