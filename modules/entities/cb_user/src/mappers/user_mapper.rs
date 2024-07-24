use diesel::prelude::*;
use diesel::result::Error;
use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection};
use crab_rocket_schema::schema::user_table::{self};
use crab_rocket_schema::schema::user_table::dsl;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

use crate::models::user::{PatchUser, PostUser, User};
use crate::models::user_filter::UserFilter;

pub struct UserMapper {}

impl MapperCRUD for UserMapper {
    type Item = User;
    type PostItem = PostUser;
    type PatchItem = PatchUser;
    type Filter = UserFilter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<User>>, Error> {
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
        let total_count = dsl::user_table.count().get_result::<i64>(&mut conn)? as i32;
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
            .load::<User>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<User>, diesel::result::Error> {
        let mut conn: diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        > = establish_pg_connection(pool).expect("msg");
        let data: User = dsl::user_table.filter(dsl::user_id.eq(pid)).first(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn add_single(
        pool: &State<DbPool>,
        obj: &PostUser,
    ) -> Result<Data<User>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data = diesel::insert_into(dsl::user_table)
            .values(obj)
            .returning(User::as_returning())
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Data<User>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data: User =
            diesel::delete(dsl::user_table.filter(dsl::user_id.eq(pid))).get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchUser,
    ) -> Result<Data<User>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        let data: User = diesel::update(dsl::user_table.filter(dsl::user_id.eq(pid)))
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
            .get_result(&mut conn)?;
        Ok(Data::new(data, None))
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<User>>, diesel::result::Error> {
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
        let filter = &param.filter;
        println!("{filter:?}");
        // 计算分页相关
        let pagination = param.pagination.unwrap_or_default().clone();

        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
        // 获取总记录数
        let total_count = dsl::user_table.count().get_result::<i64>(&mut conn)? as i32;
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

        let data = query.load::<User>(&mut conn)?;
        let body = Data::new(data, Some(pagination));
        Ok(body)
    }
}

impl UserMapper {
    pub fn get_by_username(pool: &State<DbPool>, uname: String) -> Result<User, Error> {
        let mut conn = establish_pg_connection(pool).expect("msg");
        dsl::user_table.filter(dsl::username.eq(uname)).first(&mut conn)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::request_param::RequestParam;

    use crate::mappers::user_mapper::UserMapper;
    use crate::models::user::{PatchUser, PostUser};

    use super::*;

    #[test]
    fn test_insert_user() {
        let user = PostUser::demo();
        println!("{user:?}");
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::add_single(pool, &user) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("Error inserting user: {:?}", e),
        }
    }

    #[test]
    fn test_fetch_all_users() {
        let param = RequestParam::new(None, None);

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error fetching all users: {:?}", e),
        }
    }

    #[test]
    fn test_fetch_user_by_id() {
        let id = 1; // Adjust this ID based on your test data

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::get_by_id(pool, id) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error fetching user by ID: {:?}", e),
        }
    }

    #[test]
    fn test_update_user_by_id() {
        let id = 4; // Adjust this ID based on your test data
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

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::update_by_id(pool, id, &user) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error updating user by ID: {:?}", e),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        let id = 5; // Adjust this ID based on your test data

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::delete_by_id(pool, id) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error deleting user by ID: {:?}", e),
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
        let param = RequestParam::new(None, Some(filter));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserMapper::filter(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error filtering users: {:?}", e),
        }
    }
}
