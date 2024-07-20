use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::shipment_controller::ShipmentController;
use crate::models::shipment::{PatchShipment, PostShipment};
use crate::models::shipment_filter::ShipmentFilter;

#[get("/shipment?<limit>&<offset>")]
pub fn get_shipments(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ShipmentController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/shipment/filter", data = "<param>")]
pub fn filter_shipments(
    param: Option<Json<RequestParam<PaginationParam, ShipmentFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ShipmentController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/shipment/<id>")]
pub fn get_shipment_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = ShipmentController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/shipment", data = "<shipment>")]
pub fn insert_single_shipment(shipment: Json<PostShipment>) -> Json<serde_json::Value> {
    let mut obj: PostShipment = shipment.into_inner();

    let resp = ShipmentController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/shipment/<id>")]
pub fn delete_shipment_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = ShipmentController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/shipment/<id>", data = "<task>")]
pub fn update_shipment_by_id(id: i32, task: Json<PatchShipment>) -> Json<serde_json::Value> {
    let resp = ShipmentController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/shipment")]
pub fn options_shipment() -> Status {
    Status::Ok
}
