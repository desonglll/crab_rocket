use crate::models::category::{Category, PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;
use crate::services::category_service::CategoryService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct CategoryController {}

impl ControllerCRUD for CategoryController {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Param = RequestParam<CategoryFilter>;
    fn get_all(
        param: &RequestParam<CategoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Category>>>, Box<dyn Error>> {
        controller_get_all::<Category, CategoryService, CategoryFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Category>, Box<dyn Error>> {
        controller_get_by_id::<Category, CategoryService>(pid)
    }

    fn add_single(obj: &mut PostCategory) -> Result<ApiResponse<Category>, Box<dyn Error>> {
        controller_add_single::<Category, CategoryService, PostCategory>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Category>, Box<dyn Error>> {
        controller_delete_by_id::<Category, CategoryService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchCategory,
    ) -> Result<ApiResponse<Category>, Box<dyn Error>> {
        controller_update_by_id::<Category, CategoryService, PatchCategory>(pid, obj)
    }
    fn filter(
        param: &RequestParam<CategoryFilter>,
    ) -> Result<ApiResponse<Data<Vec<Category>>>, Box<dyn std::error::Error>> {
        controller_filter::<Category, CategoryService, CategoryFilter>(param)
    }
}
