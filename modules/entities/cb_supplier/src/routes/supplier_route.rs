use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::supplier_controller::SupplierController;
use crate::models::supplier::{PatchSupplier, PostSupplier};
use crate::models::supplier_filter::SupplierFilter;

#[get("/supplier?<limit>&<offset>")]
pub fn get_suppliers(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = SupplierController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/supplier/filter", data = "<param>")]
pub fn filter_suppliers(
    param: Option<Json<RequestParam<PaginationParam, SupplierFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = SupplierController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/supplier/<id>")]
pub fn get_supplier_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = SupplierController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/supplier", data = "<supplier>")]
pub fn insert_single_supplier(supplier: Json<PostSupplier>) -> Json<serde_json::Value> {
    let mut obj: PostSupplier = supplier.into_inner();

    let resp = SupplierController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/supplier/<id>")]
pub fn delete_supplier_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = SupplierController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/supplier/<id>", data = "<task>")]
pub fn update_supplier_by_id(id: i32, task: Json<PatchSupplier>) -> Json<serde_json::Value> {
    let resp = SupplierController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/supplier")]
pub fn options_supplier() -> Status {
    Status::Ok
}
