use crate::{mappers::post_mapper, models::post::Post};
use crab_rocket_schema::establish_pg_connection;

impl crate::services::post_service::GetPost for Post {
    // GOOD:
    fn insert_single_post(
        post: &crate::models::post::NewPost,
    ) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::insert_post(&mut conn, &post) {
                    Ok(inserted_post) => Ok(inserted_post),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    // GOOD:
    fn get_all_posts() -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::fetch_all_posts(&mut conn) {
                    Ok(all_posts) => Ok(all_posts),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_post_by_id(id: i32) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::fetch_post_by_id(&mut conn, id) {
                    Ok(post) => Ok(post),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn update_post_by_id(
        id: i32,
        post: &crate::models::post::PatchPost,
    ) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::update_post_by_id(&mut conn, id, post) {
                    Ok(updated_post) => Ok(updated_post),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_post_by_id(id: i32) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::delete_post_by_id(&mut conn, id) {
                    Ok(deleted_post) => Ok(deleted_post),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn filter_posts_by_params(
        params: &crate::routes::post_param::PostParam,
    ) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::fetch_posts_by_params(&mut conn, params) {
                    Ok(filtered_post) => Ok(filtered_post),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_count() -> Result<i64, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match post_mapper::get_count(&mut conn) {
                    Ok(count) => Ok(count),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
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
    #[test]
    fn test_get_all_posts() {
        use crate::models::post::Post;
        use crate::services::post_service::GetPost;
        match Post::get_all_posts() {
            Ok(all_posts) => {
                println!("{all_posts:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_get_post_by_id() {
        use crate::models::post::Post;
        use crate::services::post_service::GetPost;
        match Post::get_post_by_id(4) {
            Ok(post) => {
                println!("{post:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_delete_post_by_id() {
        use crate::models::post::Post;
        use crate::services::post_service::GetPost;
        match Post::delete_post_by_id(4) {
            Ok(deleted_post) => {
                println!("{deleted_post:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_get_posts_by_params() {
        use crate::models::post::Post;
        use crate::routes::models::post_param::PostParam;
        use crate::services::post_service::GetPost;
        let params = PostParam {
            user_id: Some(2),
            offset: Some(0),
            limit: Some(0),
        };
        match Post::filter_posts_by_params(&params) {
            Ok(posts) => {
                println!("{posts:?}");
            }
            Err(_) => (),
        }
    }
}
