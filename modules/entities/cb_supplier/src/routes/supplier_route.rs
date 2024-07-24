use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::supplier_controller::SupplierController;
use crate::models::supplier::Supplier;
use crate::models::supplier_filter::SupplierFilter;

#[get("/supplier", data = "<param>")]
pub fn get_suppliers(
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = SupplierController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/supplier/filter", data = "<param>")]
pub fn filter_suppliers(
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = SupplierController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/supplier/<id>", data = "<param>")]
pub fn get_supplier_by_id(
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = SupplierController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/supplier", data = "<param>")]
pub fn insert_single_supplier(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = SupplierController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/supplier/<id>", data = "<param>")]
pub fn delete_supplier_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = SupplierController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/supplier/<id>", data = "<param>")]
pub fn update_supplier_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Supplier, SupplierFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = SupplierController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/supplier")]
pub fn options_supplier() -> Status {
    Status::Ok
}
