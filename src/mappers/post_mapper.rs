use crate::models::info::post_info::PostInfo;
use crate::models::post::{NewPost, PatchPost, Post};
use crate::routes::models::post_param::PostParam;
use crate::schema::posts::dsl::*; //配合下面的 `posts.filter()`
use crate::schema::posts::{self};
use crate::utils::time::get_e8_time;
use diesel::prelude::*;

// GOOD:
pub fn insert_post(conn: &mut PgConnection, post: &NewPost) -> Result<Post, diesel::result::Error> {
    match diesel::insert_into(posts::table)
        .values(post)
        .returning(Post::as_returning())
        .get_result(conn)
    {
        Ok(inserted_post) => Ok(inserted_post),
        Err(e) => Err(e),
    }
}

// GOOD:
pub fn fetch_all_posts(conn: &mut PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
    posts::table.order(posts::post_id.asc()).load::<Post>(conn)
}

// GOOD:
pub fn fetch_post_by_id(conn: &mut PgConnection, id: i32) -> Result<Post, diesel::result::Error> {
    // 配合 use crate::schema::posts::dsl::*;
    posts.filter(posts::post_id.eq(id)).first(conn)
}

// GOOD:
pub fn update_post_by_id(
    conn: &mut PgConnection,
    id: i32,
    post: &PatchPost,
) -> Result<Post, diesel::result::Error> {
    diesel::update(posts.filter(post_id.eq(id)))
        .set((
            posts::title.eq(post.title.clone()),
            posts::body.eq(post.body.clone()),
            posts::user_id.eq(post.user_id),
            posts::status.eq(post.status.clone()),
            posts::updated_at.eq(get_e8_time()),
        ))
        .get_result(conn)
}

pub fn delete_post_by_id(conn: &mut PgConnection, id: i32) -> Result<Post, diesel::result::Error> {
    diesel::delete(posts.filter(posts::post_id.eq(id))).get_result(conn)
}

pub fn fetch_posts_by_params(
    conn: &mut PgConnection,
    params: &PostParam,
) -> (Result<Vec<Post>, diesel::result::Error>, PostInfo) {
    let mut query = posts.into_boxed();

    if let Some(uid) = params.user_id {
        if uid != 0 {
            query = query.filter(posts::user_id.eq(uid));
        }
    }
    if let Some(limit) = params.limit {
        if limit != 0 {
            query = query.limit(limit.into());
        }
    }
    if let Some(offset) = params.offset {
        if offset != 0 {
            query = query.offset(offset.into());
        }
    }
    let filtered_posts = query.order(posts::post_id.asc()).load::<Post>(conn);
    let count: i64 = posts.count().first(conn).expect("Error counting posts");

    (filtered_posts, PostInfo { count: count })
}
pub fn get_count(conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
    let count: i64 = posts.count().first(conn).expect("Error counting posts");
    Ok(count)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_insert_post() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                // 创建一个新的 NewPost 实例
                let new_post = NewPost::new(
                    Some("Test Title".to_string()),
                    Some("Test Body".to_string()),
                    Some(4),
                    Some("Published".to_string()),
                    Some(chrono::Utc::now().naive_utc()), // 使用当前时间作为创建时间
                    Some(chrono::Utc::now().naive_utc()), // 使用当前时间作为更新时间
                );

                // 调用 insert_post 函数
                let _ = insert_post(&mut conn, &new_post);
                // 删除插入的数据，以便下一次测试
                // diesel::delete(posts::table.filter(posts::title.eq("Test
                // Title"))) .execute(&mut conn)
                // .expect("Failed to delete test data");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_all_posts() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_all_posts(&mut conn) {
                Ok(all_posts) => {
                    println!("{all_posts:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_post_by_id() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_post_by_id(&mut conn, 1) {
                Ok(post) => {
                    println!("{post:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_update_post_by_id() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        let new_post = NewPost::new(
            Some("Test newe".to_string()),
            Some("Test Body".to_string()),
            Some(1),
            Some("Published".to_string()),
            Some(chrono::Utc::now().naive_utc()), // 使用当前时间作为创建时间
            Some(chrono::Utc::now().naive_utc()), // 使用当前时间作为更新时间
        );
        match establish_pg_connection() {
            Ok(mut conn) => match update_post_by_id(&mut conn, 4, &new_post.into()) {
                Ok(updated_post) => {
                    println!("{updated_post:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_delete_post_by_id() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match delete_post_by_id(&mut conn, 1) {
                Ok(deleted_post) => {
                    println!("{deleted_post:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_posts_by_params() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        let params = PostParam {
            user_id: Some(2),
            limit: None,
            offset: None,
        };
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_posts_by_params(&mut conn, &params).0 {
                Ok(u_posts) => {
                    println!("{u_posts:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }
}
