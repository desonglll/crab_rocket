use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;

use crate::models::follow::{Follow, PatchFollow, PostFollow};
use crate::models::follow_filter::FollowFilter;
use crate::services::follow_service::FollowService;
use crate::services::follow_service_trait::FollowServiceTrait;

use super::follow_controller_trait::FollowControllerTrait;

pub struct FollowController {}

impl ControllerCRUD for FollowController {
    type Item = Follow;
    type PostItem = PostFollow;
    type PatchItem = PatchFollow;
    type Param = RequestParam<FollowFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<FollowFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, FollowService, FollowFilter>(pool, param)
    }

    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, FollowService>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &mut PostFollow,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, FollowService, PostFollow>(pool, obj)
    }

    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, FollowService>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, FollowService, PatchFollow>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<FollowFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, FollowService, FollowFilter>(pool, param)
    }
}

impl FollowControllerTrait<RequestParam<FollowFilter>> for FollowController {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::delete_follow_specifically(pool, obj) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<FollowFilter>,
    ) -> Result<
        ApiResponse<obj_traits::response::data::Data<Vec<Follow>>>,
        Box<dyn std::error::Error>,
    > {
        match FollowService::get_followeds_by_user_id(pool, uid, param) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<FollowFilter>,
    ) -> Result<
        ApiResponse<obj_traits::response::data::Data<Vec<Follow>>>,
        Box<dyn std::error::Error>,
    > {
        match FollowService::get_followings_by_user_id(pool, uid, param) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
