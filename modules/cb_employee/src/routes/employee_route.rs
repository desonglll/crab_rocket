use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use serde_json::json;

use super::employee_param::EmployeeParam;
use crate::{
    controllers::employee_controller,
    models::employee::{NewEmployee, PatchEmployee},
};

// for employee insert
#[utoipa::path(
    responses(
        (status = 200, description = "created successfully", body = Employee),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/employee", data = "<employee>")]
pub fn insert_single_employee(employee: Json<NewEmployee>) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let (code, message, inserted_employee) =
        employee_controller::insert_single_employee_controller(&employee);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":inserted_employee
    }))
    .unwrap();
    Json(response)
}
#[get("/employee")]
pub fn get_all_employees() -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let (code, message, all_employees) = employee_controller::get_all_employees_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":all_employees
    }))
    .unwrap();
    Json(response)
}
// for employee delete
#[utoipa::path(
    responses(
        (status = 200, description = "delete successfully", body = Employee),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[delete("/employee/<id>")]
pub fn delete_employee_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, updated_user) = employee_controller::delete_employee_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_user
    }))
    .unwrap();
    Json(response)
}

// for employee filter by params
#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Vec<Employee>),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/employee/filter", data = "<params>")]
pub fn get_employee_by_params(params: Json<EmployeeParam>) -> Json<serde_json::Value> {
    let (code, message, emp) = employee_controller::get_employee_by_params_controller(&params);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":emp,
    }))
    .unwrap();
    Json(response)
}

#[get("/employee/<id>")]
pub fn get_employee_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, result) = employee_controller::get_employee_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":result
    }))
    .unwrap();
    Json(response)
}
#[utoipa::path(
    responses(
        (status = 200, description = "update successfully", body = Employee),
        (status = NOT_FOUND, description = "not found")     )
)]
#[patch("/employee/<id>", data = "<emp>")]
pub fn update_employee_by_id(id: i32, emp: Json<PatchEmployee>) -> Json<serde_json::Value> {
    let (code, message, updated_emp) =
        employee_controller::update_employee_by_id_controller(id, &emp);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_emp
    }))
    .unwrap();
    Json(response)
}
#[options("/employee/filter")]
pub fn options_employee_filter() -> Status {
    Status::Ok
}
