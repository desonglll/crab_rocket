use rocket::{delete, get, post, serde::json::Json};
use serde_json::json;

use crate::{controllers::employee_controller, models::employee::NewEmployee};

use super::models::employee_param::EmployeeParam;
#[post("/employee", data = "<employee>")]
pub fn insert_single_employee(employee: Json<NewEmployee>) -> Json<serde_json::Value> {
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

#[post("/employee/filter", data = "<params>")]
pub fn get_employee_by_params(params: Json<EmployeeParam>) -> Json<serde_json::Value> {
    let (code, message, (emp, info)) =
        employee_controller::get_employee_by_params_controller(&params);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":{
            "posts":emp,
            "info":info
        }
    }))
    .unwrap();
    Json(response)
}
