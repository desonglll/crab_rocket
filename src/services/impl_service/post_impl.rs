use crate::{establish_pg_connection, mappers::post_mapper, models::post::Post};

impl crate::services::post_service::GetPost for Post {
    fn insert_single_post(
        post: &crate::models::post::NewPost,
    ) -> Result<Post, diesel::result::Error> {
        match establish_pg_connection() {
            Ok(mut conn) => match post_mapper::insert_post(&mut conn, &post) {
                Ok(inserted_post) => Ok(inserted_post),
                Err(e) => Err(e),
            },
            Err(_) => Err(diesel::result::Error::NotFound),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_insert_single_post() {
        use crate::models::post::{NewPost, Post};
        use crate::services::post_service::GetPost;
        let post = NewPost::new(
            Some("Test insert_post".to_string()),
            Some("Body".to_string()),
            Some(1),
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
        let inserted_post = Post::insert_single_post(&post.into());
        println!("{inserted_post:?}");
    }
}
