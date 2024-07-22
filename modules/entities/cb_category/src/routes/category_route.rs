use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::State;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::category_controller::CategoryController;
use crate::models::category::{PatchCategory, PostCategory};
use crate::models::category_filter::CategoryFilter;

#[get("/category?<limit>&<offset>")]
pub fn get_categorys(
    pool: &State<DbPool>,
    mut limit: Option<i32>,
    mut offset: Option<i32>,
) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = CategoryController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/category/filter", data = "<param>")]
pub fn filter_categorys(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<CategoryFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = CategoryController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/category/<id>")]
pub fn get_category_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = CategoryController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/category", data = "<category>")]
pub fn insert_single_category(
    pool: &State<DbPool>,
    category: Json<PostCategory>,
) -> Json<serde_json::Value> {
    let mut obj: PostCategory = category.into_inner();

    let resp = CategoryController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/category/<id>")]
pub fn delete_category_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = CategoryController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/category/<id>", data = "<task>")]
pub fn update_category_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchCategory>,
) -> Json<serde_json::Value> {
    let resp = CategoryController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/category")]
pub fn options_category() -> Status {
    Status::Ok
}
