use std::error::Error;

use obj_traits::{
    controller::controller_crud::{
        controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
        controller_get_by_id, controller_update_by_id, ControllerCRUD,
    },
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    response::{api_response::ApiResponse, data::Data},
};

use crate::{
    models::{
        follow::{Follow, NewFollow, PatchFollow},
        follow_filter::FollowFilter,
    },
    services::{follow_service::FollowService, follow_service_trait::FollowServiceTrait},
};

use super::follow_controller_trait::FollowControllerTrait;

pub struct FollowController {}

impl ControllerCRUD for FollowController {
    type Item = Follow;
    type NewItem = NewFollow;
    type PatchItem = PatchFollow;
    type Param = RequestParam<PaginationParam, FollowFilter>;

    fn get_all(
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, FollowService, FollowFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, FollowService>(pid)
    }

    fn add_single(obj: &mut NewFollow) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, FollowService, NewFollow>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, FollowService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, FollowService, PatchFollow>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, FollowService, FollowFilter>(param)
    }
}

impl FollowControllerTrait<RequestParam<PaginationParam, FollowFilter>> for FollowController {
    fn delete_follow_specifically(
        obj: &NewFollow,
    ) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::delete_follow_specifically(obj) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_followeds_by_user_id(
        uid: i32,
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<
        ApiResponse<obj_traits::response::data::Data<Vec<Follow>>>,
        Box<dyn std::error::Error>,
    > {
        match FollowService::get_followeds_by_user_id(uid, param) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_followings_by_user_id(
        uid: i32,
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<
        ApiResponse<obj_traits::response::data::Data<Vec<Follow>>>,
        Box<dyn std::error::Error>,
    > {
        match FollowService::get_followings_by_user_id(uid, param) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
