use crate::mappers::category_mapper::CategoryMapper;
use crate::models::category::{Category, PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

pub struct CategoryService {}

impl ServiceCRUD for CategoryService {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Param = RequestParam<CategoryFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<CategoryFilter>,
    ) -> Result<Data<Vec<Category>>, Box<dyn Error>> {
        service_get_all::<Category, CategoryMapper, CategoryFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Category, Box<dyn Error>> {
        service_get_by_id::<Category, CategoryMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostCategory) -> Result<Category, Box<dyn Error>> {
        service_add_single::<Category, CategoryMapper, PostCategory>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Category, Box<dyn Error>> {
        service_delete_by_id::<Category, CategoryMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchCategory,
    ) -> Result<Category, Box<dyn Error>> {
        service_update_by_id::<Category, CategoryMapper, PatchCategory>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<CategoryFilter>,
    ) -> Result<Data<Vec<Category>>, Box<dyn std::error::Error>> {
        service_filter::<Category, CategoryMapper, CategoryFilter>(pool, param)
    }
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
            Ok(result) => println!("{result:?}"),
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
