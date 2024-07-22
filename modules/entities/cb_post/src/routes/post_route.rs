use crate::controllers::post_controller::PostController;
use crate::models::post::{PatchPost, PostPost};
use crate::models::post_filter::PostFilter;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, options, patch, post};
use serde_json::json;

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中
/// ## Put和Patch
/// `https://ihower.tw/blog/archives/6483`
/// PUT 比較正確的定義是 Replace (Create or Update)，
/// 例如PUT/items/1的意思是替換/items/1，如果已經存在就替換，沒有就新增。
/// PUT必須包含items/1的所有屬性資料
#[get("/post?<limit>&<offset>")]
pub fn get_posts(limit: Option<i32>, offset: Option<i32>) -> Json<serde_json::Value> {
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = PostController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/post/filter", data = "<param>")]
pub fn filter_posts(param: Option<Json<RequestParam<PostFilter>>>) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    println!("{param:?}");
    let resp = PostController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/post/<id>")]
pub fn get_post_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = PostController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/post", data = "<post>")]
pub fn insert_single_post(post: Json<PostPost>) -> Json<serde_json::Value> {
    let mut obj: PostPost = post.into_inner();

    let resp = PostController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/post/<id>")]
pub fn delete_post_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = PostController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/post/<id>", data = "<post>")]
pub fn update_post_by_id(id: i32, post: Json<PatchPost>) -> Json<serde_json::Value> {
    let resp = PostController::update_by_id(id, &post).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world!"
}

#[get("/test")]
pub fn demo() -> Json<serde_json::Value> {
    let j = json!({
        "code": 200,
        "message": "success",
        "data": {
            "user": {
              "id": 123,
              "name": "John Doe",
              "email": "john@example.com"
            }
          }
    });
    Json(serde_json::from_value(j).unwrap())
}

#[options("/post/filter")]
pub fn options_post_filter() -> Status {
    Status::Ok
}
