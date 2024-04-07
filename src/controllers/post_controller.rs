use crate::{
    models::post::{NewPost, PatchPost, Post},
    services::post_service::GetPost,
};

pub fn insert_single_post_controller(post: &NewPost) -> (i32, &'static str, Post) {
    match Post::insert_single_post(post) {
        Ok(result) => (200, "INSERT POST OK", result),
        Err(_) => (204, "INSERT POST ERROR", Post::new_empty()),
    }
}

pub fn get_all_posts_controller() -> (i32, &'static str, Vec<Post>) {
    match Post::get_all_posts() {
        Ok(all_posts) => (200, "GET ALL POSTS OK", all_posts),
        Err(_) => (204, "GET ALL POSTS EROR", Vec::new()),
    }
}

pub fn get_post_by_id_controller(id: i32) -> (i32, &'static str, Post) {
    match Post::get_post_by_id(id) {
        Ok(post) => (200, "GET POST BY ID OK", post),
        Err(_) => (204, "GET POST BY ID ERROR", Post::new_empty()),
    }
}

pub fn update_post_by_id_controller(id: i32, post: &PatchPost) -> (i32, &'static str, Post) {
    match Post::update_post_by_id(id, post) {
        Ok(updated_post) => (200, "UPDATE POST BY ID OK", updated_post),
        Err(_) => (204, "UPDATE POST BY ID ERROR", Post::new_empty()),
    }
}
pub fn delete_post_by_id_controller(id: i32) -> (i32, &'static str, Post) {
    match Post::delete_post_by_id(id) {
        Ok(deleted_post) => (200, "DELETE POST BY ID OK", deleted_post),
        Err(_) => (204, "DELETE POST BY ID ERROR", Post::new_empty()),
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_insert_single_post_controller() {
        use super::insert_single_post_controller;
        use crate::models::post::NewPost;
        let post = NewPost::new(
            Some("Title".to_string()),
            Some("Body".to_string()),
            Some(12),
            Some("Published".to_string()),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
            Some(
                chrono::NaiveDateTime::parse_from_str("2024-04-06 12:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap(),
            ),
        );
        let (code, message, inserted_post) = insert_single_post_controller(&post);
        println!("{code}\n{message}\n{inserted_post:?}\n");
    }
}
