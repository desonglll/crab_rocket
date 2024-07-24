use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::shipment_controller::ShipmentController;
use crate::models::shipment::Shipment;
use crate::models::shipment_filter::ShipmentFilter;

#[get("/shipment", data = "<param>")]
pub fn get_shipments(
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = ShipmentController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment/filter", data = "<param>")]
pub fn filter_shipments(
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = ShipmentController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment/<id>", data = "<param>")]
pub fn get_shipment_by_id(
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = ShipmentController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment", data = "<param>")]
pub fn insert_single_shipment(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = ShipmentController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/shipment/<id>", data = "<param>")]
pub fn delete_shipment_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = ShipmentController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/shipment/<id>", data = "<param>")]
pub fn update_shipment_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Shipment, ShipmentFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = ShipmentController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/shipment")]
pub fn options_shipment() -> Status {
    Status::Ok
}
