use diesel::PgConnection;
//配合下面的 `post_table.filter()`
use diesel::prelude::*;
use diesel::result::Error;

use crab_rocket_schema::schema::post_table::dsl;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

use crate::models::post::{PatchPost, Post, PostPost};
use crate::models::post_filter::PostFilter;

pub struct PostMapper {}

impl MapperCRUD for PostMapper {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Param = RequestParam<PostFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PostFilter>,
    ) -> Result<Data<Vec<Post>>, Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::post_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );

        // 分页查询
        let data = dsl::post_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Post>(conn)?;
        let resp_body = Data::new(data, pagination);
        println!("{resp_body}");
        Ok(resp_body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Post, diesel::result::Error> {
        // 配合 use crate::schema::post_table::dsl::*;
        dsl::post_table.filter(dsl::post_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &PostPost) -> Result<Post, diesel::result::Error> {
        match diesel::insert_into(dsl::post_table)
            .values(obj)
            .returning(Post::as_returning())
            .get_result(conn)
        {
            Ok(inserted_post) => Ok(inserted_post),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Post, diesel::result::Error> {
        diesel::delete(dsl::post_table.filter(dsl::post_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchPost,
    ) -> Result<Post, diesel::result::Error> {
        diesel::update(dsl::post_table.filter(dsl::post_id.eq(pid)))
            .set((
                dsl::title.eq(&obj.title),
                dsl::body.eq(&obj.body),
                dsl::user_id.eq(obj.user_id),
                dsl::status.eq(&obj.status),
                dsl::created_at.eq(obj.created_at),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PostFilter>,
    ) -> Result<Data<Vec<Post>>, diesel::result::Error> {
        // 当前页码（page）
        // 每页条目数（per_page）
        //
        // 总页数（total_pages）
        //
        // 公式
        //
        // 当前页的 offset: (page - 1) * per_page
        //
        // 下一页的 offset: page * per_page
        //
        // 上一页的 offset: (page - 2) * per_page （如果 page > 1）
        //
        // limit 始终为 per_page
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::post_table.count().get_result::<i64>(conn)? as i32;
        // 计算总页数
        let total_pages = (total_count + per_page - 1) / per_page;

        let previous_page_offset = (page - 2) * per_page;
        let next_page_offset = page * per_page;
        let pagination = Pagination::new(
            page,
            per_page,
            total_pages,
            total_count,
            Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
            Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
        );
        let mut query = dsl::post_table.into_boxed();
        let filter = &param.filter;
        println!("{filter:?}");
        if let Some(f) = filter {
            if let Some(post_id) = &f.post_id {
                query = query.filter(dsl::post_id.eq(post_id));
            }
            if let Some(title) = &f.title {
                query = query.filter(dsl::title.like(format!("%{}%", title)));
            }
            if let Some(body) = &f.body {
                query = query.filter(dsl::body.like(format!("%{}%", body)));
            }
            if let Some(user_id) = &f.user_id {
                query = query.filter(dsl::user_id.eq(user_id));
            }
            if let Some(status) = &f.status {
                query = query.filter(dsl::status.eq(status));
            }
            if let Some(created_at_min) = &f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = &f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(updated_at_min) = &f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = &f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
            if let Some(username) = &f.username {
                query = query.filter(dsl::username.like(format!("%{}%", username)));
            }
        }

        let data = query.load::<Post>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}
#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pg_connection, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};

    use super::*;

    #[test]
    fn test_insert_post() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        // 創建一個新的 PostPost 實例
        let new_post = PostPost::demo();

        // 插入新 Post
        let inserted_post =
            PostMapper::add_single(&mut conn, &new_post).expect("Failed to insert post");

        // 確認插入的 Post 的 title 是否符合預期
        assert_eq!(inserted_post.title, new_post.title);
    }

    #[test]
    fn test_fetch_all_post_table() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let param = RequestParam::new(Some(PaginationParam::demo()), None);

        // 查詢所有 Post 記錄
        let result = PostMapper::get_all(&mut conn, &param).expect("Failed to fetch all posts");
        // 確認結果是否正確
        assert!(result.data().len() > 0); // 確認返回了至少一條記錄
    }

    #[test]
    fn test_fetch_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post =
            PostMapper::add_single(&mut conn, &new_post).expect("Failed to insert post");

        // 查詢 Post 根據 ID
        let fetched_post =
            PostMapper::get_by_id(&mut conn, inserted_post.post_id).expect("Failed to fetch post");

        // 確認查詢結果是否與插入的 Post 匹配
        assert_eq!(fetched_post.title, new_post.title);
    }

    #[test]
    fn test_update_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post =
            PostMapper::add_single(&mut conn, &new_post).expect("Failed to insert post");

        // 更新 Post
        let updated_post_data =
            PatchPost::new(Some("Updated Title".to_string()), None, None, None, None, None);
        let updated_post =
            PostMapper::update_by_id(&mut conn, inserted_post.post_id, &updated_post_data)
                .expect("Failed to update post");

        // 確認更新結果
        assert_eq!(updated_post.title, updated_post_data.title);
    }

    #[test]
    fn test_delete_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post =
            PostMapper::add_single(&mut conn, &new_post).expect("Failed to insert post");

        // 刪除 Post
        let deleted_post = PostMapper::delete_by_id(&mut conn, inserted_post.post_id)
            .expect("Failed to delete post");

        // 確認刪除結果
        assert_eq!(deleted_post.post_id, inserted_post.post_id);

        // 確認刪除後記錄是否不存在
        let fetched_post = PostMapper::get_by_id(&mut conn, inserted_post.post_id);
        assert!(fetched_post.is_err());
    }
}
