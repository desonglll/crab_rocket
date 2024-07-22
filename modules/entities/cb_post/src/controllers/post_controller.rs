use std::error::Error;

use obj_traits::{
    controller::controller_crud::{
        controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
        controller_get_by_id, controller_update_by_id, ControllerCRUD,
    },
    request::request_param::RequestParam,
    response::{api_response::ApiResponse, data::Data},
};

use crate::{
    models::{
        post::{PatchPost, Post, PostPost},
        post_filter::PostFilter,
    },
    services::post_service::PostService,
};

pub struct PostController {}

impl ControllerCRUD for PostController {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Param = RequestParam<PostFilter>;
    fn get_all(
        param: &RequestParam<PostFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, PostService, PostFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, PostService>(pid)
    }

    fn add_single(obj: &mut PostPost) -> Result<ApiResponse<Post>, Box<dyn Error>> {
        controller_add_single::<Self::Item, PostService, PostPost>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, PostService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchPost) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, PostService, PatchPost>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PostFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, PostService, PostFilter>(param)
    }
}
