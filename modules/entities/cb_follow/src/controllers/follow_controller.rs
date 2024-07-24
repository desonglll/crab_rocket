use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
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
    type Filter = FollowFilter;
    type Service = FollowService;
}

impl FollowControllerTrait<RequestParam<Follow, FollowFilter>> for FollowController {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<ApiResponse<Data<Follow>>, Box<dyn std::error::Error>> {
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
        param: &RequestParam<Follow, FollowFilter>,
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
        param: &RequestParam<Follow, FollowFilter>,
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
