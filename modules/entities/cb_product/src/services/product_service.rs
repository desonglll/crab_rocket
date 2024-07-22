use crate::mappers::product_mapper::ProductMapper;
use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

pub struct ProductService {}

impl ServiceCRUD for ProductService {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Param = RequestParam<ProductFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<ProductFilter>,
    ) -> Result<Data<Vec<Product>>, Box<dyn Error>> {
        service_get_all::<Product, ProductMapper, ProductFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Product, Box<dyn Error>> {
        service_get_by_id::<Product, ProductMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostProduct) -> Result<Product, Box<dyn Error>> {
        service_add_single::<Product, ProductMapper, PostProduct>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Product, Box<dyn Error>> {
        service_delete_by_id::<Product, ProductMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchProduct,
    ) -> Result<Product, Box<dyn Error>> {
        service_update_by_id::<Product, ProductMapper, PatchProduct>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<ProductFilter>,
    ) -> Result<Data<Vec<Product>>, Box<dyn std::error::Error>> {
        service_filter::<Product, ProductMapper, ProductFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::product_service::ProductService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_product() {
        use crate::models::product::PostProduct;
        let product = PostProduct::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ProductService::add_single(pool, &product) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_products() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ProductService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_product_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ProductService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_product_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ProductService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
