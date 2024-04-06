use crate::models::post::{NewPost, Post};

pub trait GetPost {
    fn insert_single_post(post: &NewPost) -> Result<Post, diesel::result::Error>;
}
