use obj_traits::{
    controller::controller_crud::ControllerCRUD,
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    response::api_response::ApiResponse,
    service::service_crud::ServiceCRUD,
};

use crate::{
    models::follow::{Follow, NewFollow, PatchFollow},
    services::{follow_service::FollowService, follow_service_trait::FollowServiceTrait},
};

use super::follow_controller_trait::FollowControllerTrait;

pub struct FollowController {}

impl ControllerCRUD<Follow, NewFollow, PatchFollow, RequestParam<PaginationParam>>
    for FollowController
{
    fn get_all(
        param: &RequestParam<PaginationParam>,
    ) -> Result<
        obj_traits::response::api_response::ApiResponse<
            obj_traits::response::data::Data<Vec<Follow>>,
        >,
        Box<dyn std::error::Error>,
    > {
        match FollowService::get_all(param) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::get_by_id(pid) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn add_single(obj: &mut NewFollow) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::add_single(obj) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::delete_by_id(pid) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
    fn update_by_id(
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>> {
        match FollowService::update_by_id(pid, obj) {
            Ok(data) => Ok(ApiResponse::success(data)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
impl FollowControllerTrait<RequestParam<PaginationParam>> for FollowController {
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
        param: &RequestParam<PaginationParam>,
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
        param: &RequestParam<PaginationParam>,
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
