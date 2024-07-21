use crate::models::user::{PatchUser, PostUser, User};
use crate::models::user_filter::UserFilter;
use crab_rocket_schema::schema::user_table::dsl;
use crab_rocket_schema::schema::user_table::{self};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use diesel::result::Error;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

pub struct UserMapper {}

impl MapperCRUD for UserMapper {
    type Item = User;
    type PostItem = PostUser;
    type PatchItem = PatchUser;
    type Param = RequestParam<PaginationParam, UserFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, Error> {
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
        let total_count = dsl::user_table.count().get_result::<i64>(conn)? as i32;
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
        let data = dsl::user_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<User>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<User, Error> {
        dsl::user_table.filter(dsl::user_id.eq(pid)).first(conn)
    }
    fn add_single(conn: &mut PgConnection, obj: &PostUser) -> Result<User, Error> {
        diesel::insert_into(dsl::user_table)
            .values(obj)
            .returning(User::as_returning())
            .get_result(conn)
    }
    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<User, Error> {
        diesel::delete(dsl::user_table.filter(dsl::user_id.eq(pid))).get_result(conn)
    }
    fn update_by_id(conn: &mut PgConnection, pid: i32, obj: &PatchUser) -> Result<User, Error> {
        diesel::update(dsl::user_table.filter(dsl::user_id.eq(pid)))
            .set((
                user_table::username.eq(&obj.username),
                user_table::password.eq(&obj.password),
                user_table::role_id.eq(obj.role_id),
                user_table::email.eq(&obj.email),
                user_table::full_name.eq(&obj.full_name),
                user_table::avatar_url.eq(&obj.avatar_url),
                user_table::bio.eq(&obj.bio),
                user_table::updated_at.eq(Some(get_e8_time())),
                user_table::mobile_phone.eq(&obj.mobile_phone),
                user_table::created_at.eq(obj.created_at),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, diesel::result::Error> {
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
        let filter = &param.filter;
        println!("{filter:?}");
        // 计算分页相关
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::user_table.count().get_result::<i64>(conn)? as i32;
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

        let mut query = dsl::user_table.into_boxed();

        if let Some(f) = filter {
            if let Some(username) = &f.username {
                query = query.filter(dsl::username.like(format!("%{}%", username)));
            }
            if let Some(role_id) = &f.role_id {
                query = query.filter(dsl::role_id.eq(role_id));
            }
            if let Some(created_at_min) = &f.created_at_min {
                query = query.filter(dsl::created_at.ge(created_at_min));
            }
            if let Some(created_at_max) = &f.created_at_max {
                query = query.filter(dsl::created_at.le(created_at_max));
            }
            if let Some(email) = &f.email {
                query = query.filter(dsl::email.like(format!("%{}%", email)));
            }
            if let Some(full_name) = &f.full_name {
                query = query.filter(dsl::full_name.like(format!("%{}%", full_name)));
            }
            if let Some(avatar_url) = &f.avatar_url {
                query = query.filter(dsl::avatar_url.like(format!("%{}%", avatar_url)));
            }
            if let Some(bio) = &f.bio {
                query = query.filter(dsl::bio.like(format!("%{}%", bio)));
            }
            if let Some(updated_at_min) = &f.updated_at_min {
                query = query.filter(dsl::updated_at.ge(updated_at_min));
            }
            if let Some(updated_at_max) = &f.updated_at_max {
                query = query.filter(dsl::updated_at.le(updated_at_max));
            }
            if let Some(mobile_phone) = &f.mobile_phone {
                query = query.filter(dsl::mobile_phone.like(format!("%{}%", mobile_phone)));
            }
        }

        let data = query.load::<User>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mappers::user_mapper::UserMapper;
    use crate::models::user::{PatchUser, PostUser};
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;

    #[test]
    fn test_insert_user() {
        let user = PostUser::demo();
        println!("{user:?}");
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::add_single(&mut conn, &user) {
                Ok(result) => println!("{result}"),
                Err(e) => println!("Error inserting user: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }

    #[test]
    fn test_fetch_all_users() {
        let param = RequestParam::new(PaginationParam::demo(), None);

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_all(&mut conn, &param) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("Error fetching all users: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }

    #[test]
    fn test_fetch_user_by_id() {
        let id = 1; // Adjust this ID based on your test data

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_by_id(&mut conn, id) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("Error fetching user by ID: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }

    #[test]
    fn test_update_user_by_id() {
        let id = 1; // Adjust this ID based on your test data
        let user = PatchUser {
            username: "updated_username".to_string(),
            role_id: Some(2),
            created_at: None,
            email: Some("updated_email@example.com".to_string()),
            password: "updated_password".to_string(),
            full_name: Some("Updated Fullname".to_string()),
            avatar_url: Some("https://example.com/updated_avatar.jpg".to_string()),
            bio: Some("Updated bio".to_string()),
            updated_at: Some(get_e8_time()),
            mobile_phone: "0987654321".to_string(),
        };

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::update_by_id(&mut conn, id, &user) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("Error updating user by ID: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        let id = 1; // Adjust this ID based on your test data

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::delete_by_id(&mut conn, id) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("Error deleting user by ID: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }

    #[test]
    fn test_filter_users() {
        let filter = UserFilter {
            username: Some("username".to_string()), // Adjust filter criteria as needed
            role_id: None,
            created_at_min: None,
            created_at_max: None,
            email: None,
            full_name: None,
            avatar_url: None,
            bio: None,
            updated_at_min: None,
            updated_at_max: None,
            mobile_phone: None,
            offset: Some(0),
            limit: Some(10),
        };
        let param = RequestParam::new(PaginationParam::demo(), Some(filter));

        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::filter(&mut conn, &param) {
                Ok(res) => println!("{res}"),
                Err(e) => println!("Error filtering users: {:?}", e),
            },
            Err(e) => println!("establish_pg_connection error: {:?}", e),
        }
    }
}
