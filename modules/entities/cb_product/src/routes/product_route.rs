use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::product_controller::ProductController;
use crate::models::product::{PatchProduct, PostProduct};
use crate::models::product_filter::ProductFilter;

#[get("/product?<limit>&<offset>")]
pub fn get_products(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ProductController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/product/filter", data = "<param>")]
pub fn filter_products(
    param: Option<Json<RequestParam<PaginationParam, ProductFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ProductController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/product/<id>")]
pub fn get_product_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ProductController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/product", data = "<product>")]
pub fn insert_single_product(product: Json<PostProduct>) -> Json<serde_json::Value> {
    let mut obj: PostProduct = product.into_inner();

    let resp = ProductController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/product/<id>")]
pub fn delete_product_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = ProductController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/product/<id>", data = "<task>")]
pub fn update_product_by_id(id: i32, task: Json<PatchProduct>) -> Json<serde_json::Value> {
    let resp = ProductController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/product")]
pub fn options_product() -> Status {
    Status::Ok
}
