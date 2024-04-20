use crate::{
    models::post::{NewPost, PatchPost, Post},
    routes::models::post_param::PostParam,
    services::post_service::GetPost,
};

pub fn insert_single_post_controller(post: &NewPost) -> (i32, String, Post) {
    match Post::insert_single_post(post) {
        Ok(result) => (200, String::from("INSERT POST OK"), result),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Post::default())
        }
    }
}

pub fn get_all_posts_controller() -> (i32, String, Vec<Post>) {
    match Post::get_all_posts() {
        Ok(all_posts) => (200, String::from("GET ALL POSTS OK"), all_posts),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Vec::new())
        }
    }
}

pub fn get_post_by_id_controller(id: i32) -> (i32, String, Post) {
    match Post::get_post_by_id(id) {
        Ok(post) => (200, String::from("GET POST BY ID OK"), post),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Post::default())
        }
    }
}

pub fn update_post_by_id_controller(id: i32, post: &PatchPost) -> (i32, String, Post) {
    match Post::update_post_by_id(id, post) {
        Ok(updated_post) => (200, String::from("UPDATE POST BY ID OK"), updated_post),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Post::default())
        }
    }
}
pub fn delete_post_by_id_controller(id: i32) -> (i32, String, Post) {
    match Post::delete_post_by_id(id) {
        Ok(deleted_post) => (200, String::from("DELETE POST BY ID OK"), deleted_post),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Post::default())
        }
    }
}

pub fn get_post_by_params_controller(params: &PostParam) -> (i32, String, Vec<Post>) {
    match Post::filter_posts_by_params(params) {
        Ok(filtered_posts) => (200, String::from("GET POST BY PARAMS OK"), filtered_posts),
        Err(e) => {
            println!("{e:?}");
            (204, String::from("GET POST BY PARAMS ERROR"), Vec::new())
        }
    }
}
