use crate::models::post::{PatchPost, Post, PostPost};
use crate::models::post_filter::PostFilter;
use crate::services::post_service::PostService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct PostController {}

impl ControllerCRUD for PostController {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Filter = PostFilter;
    type Service = PostService;
}
