use obj_traits::service::service_crud::ServiceCRUD;

use crate::mappers::product_mapper::ProductMapper;
use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;

pub struct ProductService {}

impl ServiceCRUD for ProductService {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Filter = ProductFilter;
    type Mapper = ProductMapper;
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::product_service::ProductService;

    #[test]
    fn test_insert_single_product() {
        use crate::models::product::PostProduct;
        let product = PostProduct::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match ProductService::add_single(pool, &product) {
            Ok(result) => println!("{result}"),
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
