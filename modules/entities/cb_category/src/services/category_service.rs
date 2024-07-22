use crate::mappers::category_mapper::CategoryMapper;
use crate::models::category::{Category, PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct CategoryService {}

impl ServiceCRUD for CategoryService {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Param = RequestParam<CategoryFilter>;
    fn get_all(
        param: &RequestParam<CategoryFilter>,
    ) -> Result<Data<Vec<Category>>, Box<dyn Error>> {
        service_get_all::<Category, CategoryMapper, CategoryFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Category, Box<dyn Error>> {
        service_get_by_id::<Category, CategoryMapper>(pid)
    }

    fn add_single(obj: &PostCategory) -> Result<Category, Box<dyn Error>> {
        service_add_single::<Category, CategoryMapper, PostCategory>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Category, Box<dyn Error>> {
        service_delete_by_id::<Category, CategoryMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchCategory) -> Result<Category, Box<dyn Error>> {
        service_update_by_id::<Category, CategoryMapper, PatchCategory>(pid, obj)
    }
    fn filter(
        param: &RequestParam<CategoryFilter>,
    ) -> Result<Data<Vec<Category>>, Box<dyn std::error::Error>> {
        service_filter::<Category, CategoryMapper, CategoryFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use crate::services::category_service::CategoryService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_category() {
        use crate::models::category::PostCategory;
        let category = PostCategory::default();
        match CategoryService::add_single(&category) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_categorys() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        match CategoryService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_category_by_id() {
        match CategoryService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_category_by_id() {
        match CategoryService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
