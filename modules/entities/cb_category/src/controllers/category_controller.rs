use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::category::{Category, PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;
use crate::services::category_service::CategoryService;

pub struct CategoryController {}

impl ControllerCRUD for CategoryController {
    type Item = Category;
    type PostItem = PostCategory;
    type PatchItem = PatchCategory;
    type Filter = CategoryFilter;
    type Service = CategoryService;
}
