use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::customer_controller::CustomerController;
use crate::models::customer::{PatchCustomer, PostCustomer};
use crate::models::customer_filter::CustomerFilter;

#[get("/customer?<limit>&<offset>")]
pub fn get_customers(limit: Option<i32>, offset: Option<i32>) -> Json<serde_json::Value> {
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CustomerController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}
#[post("/customer/filter", data = "<param>")]
pub fn filter_customers(
    param: Option<Json<RequestParam<CustomerFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CustomerController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/customer/<id>")]
pub fn get_customer_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = CustomerController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/customer", data = "<customer>")]
pub fn insert_single_customer(customer: Json<PostCustomer>) -> Json<serde_json::Value> {
    let mut obj: PostCustomer = customer.into_inner();

    let resp = CustomerController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/customer/<id>")]
pub fn delete_customer_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = CustomerController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/customer/<id>", data = "<task>")]
pub fn update_customer_by_id(id: i32, task: Json<PatchCustomer>) -> Json<serde_json::Value> {
    let resp = CustomerController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/customer")]
pub fn options_customer() -> Status {
    Status::Ok
}
