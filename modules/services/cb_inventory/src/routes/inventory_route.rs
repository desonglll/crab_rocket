use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::State;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::inventory_controller::InventoryController;
use crate::models::inventory::{PatchInventory, PostInventory};
use crate::models::inventory_filter::InventoryFilter;

#[get("/inventory?<limit>&<offset>")]
pub fn get_inventorys(
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
    let resp = InventoryController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/inventory/filter", data = "<param>")]
pub fn filter_inventorys(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<InventoryFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = InventoryController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/inventory/<id>")]
pub fn get_inventory_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = InventoryController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/inventory", data = "<inventory>")]
pub fn insert_single_inventory(
    pool: &State<DbPool>,
    inventory: Json<PostInventory>,
) -> Json<serde_json::Value> {
    let mut obj: PostInventory = inventory.into_inner();

    let resp = InventoryController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/inventory/<id>")]
pub fn delete_inventory_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = InventoryController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/inventory/<id>", data = "<task>")]
pub fn update_inventory_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchInventory>,
) -> Json<serde_json::Value> {
    let resp = InventoryController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/inventory")]
pub fn options_inventory() -> Status {
    Status::Ok
}
