use crate::models::post::{PatchPost, Post, PostPost};
use crate::models::post_filter::PostFilter;
use crab_rocket_schema::schema::post_table::dsl;
use crab_rocket_schema::{establish_pg_connection, DbPool};
use crab_rocket_utils::time::get_e8_time;
//配合下面的 `post_table.filter()`
use diesel::prelude::*;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use rocket::State;
pub struct PostMapper {}

impl MapperCRUD for PostMapper {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Filter = PostFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Post>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let total_count = dsl::post_table.count().get_result::<i64>(&mut conn)? as i32;
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
            .load::<Post>(&mut conn)?;
        let resp_body = Data::new(data, Some(pagination));
        println!("{resp_body}");
        Ok(resp_body)
    }

    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Post>, diesel::result::Error> {
        // 配合 use crate::schema::post_table::dsl::*;
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = dsl::post_table.filter(dsl::post_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostPost,
    ) -> Result<Data<Post>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::post_table)
            .values(obj)
            .returning(Post::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<Post>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data =
            diesel::delete(dsl::post_table.filter(dsl::post_id.eq(pid))).get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchPost,
    ) -> Result<Data<Post>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::update(dsl::post_table.filter(dsl::post_id.eq(pid)))
            .set((
                dsl::title.eq(&obj.title),
                dsl::body.eq(&obj.body),
                dsl::user_id.eq(obj.user_id),
                dsl::status.eq(&obj.status),
                dsl::created_at.eq(obj.created_at),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Post>>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
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
        let total_count = dsl::post_table.count().get_result::<i64>(&mut conn)? as i32;
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

        let data = query.load::<Post>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}
#[cfg(test)]
mod tests {
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use rocket::State;

    use super::*;

    #[test]
    fn test_insert_post() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        // 創建一個新的 PostPost 實例
        let new_post = PostPost::demo();

        // 插入新 Post
        let inserted_post = PostMapper::add_single(pool, &new_post).expect("Failed to insert post");

        // 確認插入的 Post 的 title 是否符合預期
        assert_eq!(inserted_post.data.title, new_post.title);
    }

    #[test]
    fn test_fetch_all_post_table() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let param = RequestParam::new(Some(PaginationParam::demo()), None);

        // 查詢所有 Post 記錄
        let result = PostMapper::get_all(pool, &param).expect("Failed to fetch all posts");
        // 確認結果是否正確
        assert!(result.data.len() > 0); // 確認返回了至少一條記錄
    }

    #[test]
    fn test_fetch_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post = PostMapper::add_single(pool, &new_post).expect("Failed to insert post");

        // 查詢 Post 根據 ID
        let fetched_post =
            PostMapper::get_by_id(pool, inserted_post.data.post_id).expect("Failed to fetch post");

        // 確認查詢結果是否與插入的 Post 匹配
        assert_eq!(fetched_post.data.title, new_post.title);
    }

    #[test]
    fn test_update_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post = PostMapper::add_single(pool, &new_post).expect("Failed to insert post");

        // 更新 Post
        let updated_post_data =
            PatchPost::new(Some("Updated Title".to_string()), None, None, None, None, None);
        let updated_post =
            PostMapper::update_by_id(pool, inserted_post.data.post_id, &updated_post_data)
                .expect("Failed to update post");

        // 確認更新結果
        assert_eq!(updated_post.data.title, updated_post_data.title);
    }

    #[test]
    fn test_delete_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        // 插入測試 Post
        let new_post = PostPost::demo();
        let inserted_post = PostMapper::add_single(pool, &new_post).expect("Failed to insert post");

        // 刪除 Post
        let deleted_post = PostMapper::delete_by_id(pool, inserted_post.data.post_id)
            .expect("Failed to delete post");

        // 確認刪除結果
        assert_eq!(deleted_post.data.post_id, inserted_post.data.post_id);

        // 確認刪除後記錄是否不存在
        let fetched_post = PostMapper::get_by_id(pool, inserted_post.data.post_id);
        assert!(fetched_post.is_err());
    }
}
