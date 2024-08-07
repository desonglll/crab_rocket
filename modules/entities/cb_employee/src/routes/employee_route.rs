use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::employee_controller::EmployeeController;
use crate::models::employee::Employee;
use crate::models::employee_filter::EmployeeFilter;

#[get("/employee", data = "<param>")]
pub fn get_employees(
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = EmployeeController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee/filter", data = "<param>")]
pub fn filter_employees(
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = EmployeeController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee/<id>", data = "<param>")]
pub fn get_employee_by_id(
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = EmployeeController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/employee", data = "<param>")]
pub fn insert_single_employee(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = EmployeeController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/employee/<id>", data = "<param>")]
pub fn delete_employee_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = EmployeeController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/employee/<id>", data = "<param>")]
pub fn update_employee_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Employee, EmployeeFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = EmployeeController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/employee")]
pub fn options_employee() -> Status {
    Status::Ok
}
