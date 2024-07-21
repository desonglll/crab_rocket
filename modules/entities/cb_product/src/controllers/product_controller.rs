use crate::models::product::{PatchProduct, PostProduct, Product};
use crate::models::product_filter::ProductFilter;
use crate::services::product_service::ProductService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct ProductController {}

impl ControllerCRUD for ProductController {
    type Item = Product;
    type PostItem = PostProduct;
    type PatchItem = PatchProduct;
    type Param = RequestParam<PaginationParam, ProductFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, ProductFilter>,
    ) -> Result<ApiResponse<Data<Vec<Product>>>, Box<dyn Error>> {
        controller_get_all::<Product, ProductService, ProductFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Product>, Box<dyn Error>> {
        controller_get_by_id::<Product, ProductService>(pid)
    }

    fn add_single(obj: &mut PostProduct) -> Result<ApiResponse<Product>, Box<dyn Error>> {
        controller_add_single::<Product, ProductService, PostProduct>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Product>, Box<dyn Error>> {
        controller_delete_by_id::<Product, ProductService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchProduct) -> Result<ApiResponse<Product>, Box<dyn Error>> {
        controller_update_by_id::<Product, ProductService, PatchProduct>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, ProductFilter>,
    ) -> Result<ApiResponse<Data<Vec<Product>>>, Box<dyn std::error::Error>> {
        controller_filter::<Product, ProductService, ProductFilter>(param)
    }
}
