use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::category_controller::CategoryController;
use crate::models::category::{PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;

#[get("/category?<limit>&<offset>")]
pub fn get_categorys(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CategoryController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/category/filter", data = "<param>")]
pub fn filter_categorys(
    param: Option<Json<RequestParam<PaginationParam, CategoryFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CategoryController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/category/<id>")]
pub fn get_category_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CategoryController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/category", data = "<category>")]
pub fn insert_single_category(category: Json<PostCategory>) -> Json<serde_json::Value> {
    let mut obj: PostCategory = category.into_inner();

    let resp = CategoryController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/category/<id>")]
pub fn delete_category_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = CategoryController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/category/<id>", data = "<task>")]
pub fn update_category_by_id(id: i32, task: Json<PatchCategory>) -> Json<serde_json::Value> {
    let resp = CategoryController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/category")]
pub fn options_category() -> Status {
    Status::Ok
}
