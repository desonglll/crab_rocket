use crate::models::post::{NewPost, PatchPost, Post};
use crab_rocket_schema::schema::posts::dsl;
use crab_rocket_utils::time::get_e8_time;
//配合下面的 `posts.filter()`
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
pub struct PostMapper {}

impl MapperCRUD<Post, NewPost, PatchPost, RequestParam<PaginationParam>> for PostMapper {
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam>,
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
        let total_count = dsl::posts.count().get_result::<i64>(conn)? as i32;
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
        let data = dsl::posts
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Post>(conn)?;
        let resp_body = Data::new(data, pagination);
        println!("{resp_body}");
        Ok(resp_body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Post, diesel::result::Error> {
        // 配合 use crate::schema::posts::dsl::*;
        dsl::posts.filter(dsl::post_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &NewPost) -> Result<Post, diesel::result::Error> {
        match diesel::insert_into(dsl::posts)
            .values(obj)
            .returning(Post::as_returning())
            .get_result(conn)
        {
            Ok(inserted_post) => Ok(inserted_post),
            Err(e) => Err(e),
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Post, diesel::result::Error> {
        diesel::delete(dsl::posts.filter(dsl::post_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchPost,
    ) -> Result<Post, diesel::result::Error> {
        diesel::update(dsl::posts.filter(dsl::post_id.eq(pid)))
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
}
#[cfg(test)]
mod tests {

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
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        let param = RequestParam::new(PaginationParam::demo());
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::get_all(&mut conn, &param) {
                Ok(all_posts) => {
                    println!("{all_posts}")
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
