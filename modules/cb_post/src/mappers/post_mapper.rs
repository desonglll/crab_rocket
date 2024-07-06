use crate::models::post::{NewPost, PatchPost, Post};
use crate::models::post_filter::PostFilter;
use crab_rocket_schema::schema::post_table::dsl;
use crab_rocket_utils::time::get_e8_time;
//配合下面的 `post_table.filter()`
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
pub struct PostMapper {}

impl MapperCRUD<Post, NewPost, PatchPost, RequestParam<PaginationParam, PostFilter>>
    for PostMapper
{
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, PostFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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

    fn add_single(conn: &mut PgConnection, obj: &NewPost) -> Result<Post, diesel::result::Error> {
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
                dsl::title.eq(obj.title()),
                dsl::body.eq(obj.body()),
                dsl::user_id.eq(obj.user_id()),
                dsl::status.eq(obj.status()),
                dsl::created_at.eq(obj.created_at()),
                dsl::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, PostFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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
    use obj_traits::request::pagination_request_param::PaginationParamTrait;

    #[test]
    fn test_insert_post() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                // 创建一个新的 NewPost 实例
                let new_post = NewPost::demo();

                // 调用 insert_post 函数
                let _ = PostMapper::add_single(&mut conn, &new_post);
                // 删除插入的数据，以便下一次测试
                // diesel::delete(post_table::table.filter(post_table::title.eq("Test
                // Title"))) .execute(&mut conn)
                // .expect("Failed to delete test data");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_all_post_table() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        let param = RequestParam::new(PaginationParam::demo(), None);
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::get_all(&mut conn, &param) {
                Ok(all_post_table) => {
                    println!("{all_post_table}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_post_by_id() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::get_by_id(&mut conn, 1) {
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
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        let new_post = NewPost::demo();
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::update_by_id(&mut conn, 4, &new_post.into()) {
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
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::delete_by_id(&mut conn, 1) {
                Ok(deleted_post) => {
                    println!("{deleted_post:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }
}
