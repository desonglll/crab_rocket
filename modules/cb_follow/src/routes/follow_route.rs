use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    controller::controller_crud::ControllerCRUD,
    request::{
        pagination_request_param::{PaginationParam, PaginationParamTrait},
        request_param::RequestParam,
    },
};
use rocket::{delete, get, patch, post, serde::json::Json};

use crate::{
    controllers::{
        follow_controller::FollowController, follow_controller_trait::FollowControllerTrait,
    },
    models::{
        follow::{PostFollow, PatchFollow},
        follow_filter::FollowFilter,
    },
};

#[get("/follow?<limit>&<offset>")]
pub fn get_follows(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    let resp = FollowController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow/filter", data = "<param>")]
pub fn filter_follows(
    param: Option<Json<RequestParam<PaginationParam, FollowFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    println!("{param:?}");
    let resp = FollowController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow?<follower_id>&<follow_id>")]
pub fn insert_single_follow(follower_id: i32, follow_id: i32) -> Json<serde_json::Value> {
    let mut obj: PostFollow = PostFollow::new(follower_id, follow_id, Some(get_e8_time()));
    let resp = FollowController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow", data = "<follow>")]
pub fn insert_single_follow_by_params(follow: Json<PostFollow>) -> Json<serde_json::Value> {
    let mut obj: PostFollow = follow.into_inner();
    let resp = FollowController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/<id>")]
pub fn delete_follow_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = FollowController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/follow/<id>", data = "<follow>")]
pub fn update_follow_by_id(id: i32, follow: Json<PatchFollow>) -> Json<serde_json::Value> {
    let resp = FollowController::update_by_id(id, &follow).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/spec", data = "<follow>")]
pub fn delete_follow_specifically(follow: Json<PostFollow>) -> Json<serde_json::Value> {
    let resp = FollowController::delete_follow_specifically(&follow).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

// #[post("/follow/<uid>/followeds", data = "<param>")]
// pub fn get_followeds_by_user_id(
//     uid: i32,
//     param: Option<Json<RequestParam<PaginationParam>>>,
// ) -> Json<serde_json::Value> {
//     let resp = FollowController::get_followeds_by_user_id(uid, &param.unwrap()).unwrap();
//     let json_value = serde_json::to_value(&resp).unwrap();
//     Json(serde_json::from_value(json_value).unwrap())
// }

// #[post("/ffollow/<uid>/followings", data = "<param>")]
// pub fn get_followings_by_user_id(
//     uid: i32,
//     param: Option<Json<RequestParam<PaginationParam>>>,
// ) -> Json<serde_json::Value> {
//     let resp = FollowController::get_followings_by_user_id(uid, &param.unwrap()).unwrap();
//     let json_value = serde_json::to_value(&resp).unwrap();
//     Json(serde_json::from_value(json_value).unwrap())
// }
