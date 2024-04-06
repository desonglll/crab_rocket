use crate::{
    models::post::{NewPost, Post},
    services::post_service::GetPost,
};

pub fn insert_single_post_controller(post: &NewPost) -> (i32, &'static str, Post) {
    match Post::insert_single_post(post) {
        Ok(result) => (200, "INSERT POST OK", result),
        Err(_) => (204, "INSERT POST ERROR", Post::new_empty()),
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
