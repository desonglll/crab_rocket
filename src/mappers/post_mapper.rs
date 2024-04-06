use crate::models::post::{NewPost, Post};
// use crate::schema::posts::dsl::*; //配合下面的 `tasks.filter()`
use crate::schema::posts::{self};
use diesel::prelude::*;

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
}
