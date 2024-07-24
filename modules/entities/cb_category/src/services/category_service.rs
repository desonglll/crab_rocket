use crate::mappers::category_mapper::CategoryMapper;
use crate::models::category::{Category, PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct CategoryService {}

impl ServiceCRUD for CategoryService {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Filter = CategoryFilter;
    type Mapper = CategoryMapper;
}

#[cfg(test)]
mod test {
    use crate::services::category_service::CategoryService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_category() {
        use crate::models::category::PostCategory;
        let category = PostCategory::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryService::add_single(pool, &category) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_categorys() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_category_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_category_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match CategoryService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
