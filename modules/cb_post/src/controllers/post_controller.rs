use obj_traits::{
    controller::controller_crud::ControllerCRUD,
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    response::api_response::ApiResponse,
    service::service_crud::ServiceCRUD,
};

use crate::{
    models::{
        post::{NewPost, PatchPost, Post},
        post_filter::PostFilter,
    },
    services::post_service::PostService,
};

pub struct PostController {}

impl ControllerCRUD<Post, NewPost, PatchPost, RequestParam<PaginationParam, PostFilter>>
    for PostController
{
    fn get_all(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<
        obj_traits::response::api_response::ApiResponse<
            obj_traits::response::data::Data<Vec<Post>>,
        >,
        Box<dyn std::error::Error>,
    > {
        match PostService::get_all(param) {
            Ok(all_posts) => Ok(ApiResponse::success(all_posts)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Post>, Box<dyn std::error::Error>> {
        match PostService::get_by_id(pid) {
            Ok(post) => Ok(ApiResponse::success(post)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn add_single(obj: &mut NewPost) -> Result<ApiResponse<Post>, Box<dyn std::error::Error>> {
        match PostService::add_single(&obj) {
            Ok(result) => Ok(ApiResponse::success(result)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Post>, Box<dyn std::error::Error>> {
        match PostService::delete_by_id(pid) {
            Ok(deleted_post) => Ok(ApiResponse::success(deleted_post)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchPost,
    ) -> Result<ApiResponse<Post>, Box<dyn std::error::Error>> {
        match PostService::update_by_id(pid, obj) {
            Ok(updated_post) => Ok(ApiResponse::success(updated_post)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }

    fn filter(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<ApiResponse<obj_traits::response::data::Data<Vec<Post>>>, Box<dyn std::error::Error>>
    {
        match PostService::filter(param) {
            Ok(all_posts) => Ok(ApiResponse::success(all_posts)),
            Err(e) => {
                println!("{e:?}");
                Ok(ApiResponse::error(e))
            }
        }
    }
}
