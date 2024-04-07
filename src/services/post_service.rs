use crate::models::post::{NewPost, PatchPost, Post};

pub trait GetPost {
    fn insert_single_post(post: &NewPost) -> Result<Post, Box<dyn std::error::Error>>;
    fn get_all_posts() -> Result<Vec<Post>, Box<dyn std::error::Error>>;
    fn get_post_by_id(id: i32) -> Result<Post, Box<dyn std::error::Error>>;
    fn update_post_by_id(id: i32, post: &PatchPost) -> Result<Post, Box<dyn std::error::Error>>;
    fn delete_post_by_id(id: i32) -> Result<Post, Box<dyn std::error::Error>>;
}
