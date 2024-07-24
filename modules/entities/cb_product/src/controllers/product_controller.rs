use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;
use crate::services::product_service::ProductService;

pub struct ProductController {}

impl ControllerCRUD for ProductController {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Filter = ProductFilter;
    type Service = ProductService;
}
