use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

//配合下面的 `posts.filter()`
use crab_rocket_schema::schema::role_table::{self};
use crab_rocket_schema::schema::role_table::dsl;
use crab_rocket_utils::time::get_e8_time;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

use crate::models::role::{PatchRole, PostRole, Role};
use crate::models::role_filter::RoleFilter;

pub struct RoleMapper {}

impl MapperCRUD for RoleMapper {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Param = RequestParam<RoleFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<RoleFilter>,
    ) -> Result<Data<Vec<Role>>, Error> {
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
        let total_count = dsl::role_table.count().get_result::<i64>(conn)? as i32;
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
        let data = dsl::role_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Role>(conn)?;
        let body = Data::new(data, pagination);
        println!("{body}");
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Role, Error> {
        // 配合 use crate::schema::posts::dsl::*;
        dsl::role_table.filter(dsl::role_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &PostRole) -> Result<Role, Error> {
        match diesel::insert_into(role_table::table)
            .values(obj)
            .returning(Role::as_returning())
            .get_result(conn)
        {
            Ok(inserted_role) => Ok(inserted_role),
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Role, Error> {
        diesel::delete(dsl::role_table.filter(dsl::role_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(conn: &mut PgConnection, pid: i32, obj: &PatchRole) -> Result<Role, Error> {
        diesel::update(dsl::role_table.filter(dsl::role_id.eq(pid)))
            .set((
                role_table::role_name.eq(&obj.role_name),
                role_table::description.eq(&obj.description),
                role_table::permissions.eq(&obj.permissions),
                role_table::created_at.eq(obj.created_at),
                role_table::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<RoleFilter>,
    ) -> Result<Data<Vec<Role>>, diesel::result::Error> {
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
        let total_count = dsl::role_table.count().get_result::<i64>(conn)? as i32;
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

        let mut query = dsl::role_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(role_id) = &f.role_id {
                query = query.filter(dsl::role_id.eq(role_id));
            }
            if let Some(role_name) = &f.role_name {
                query = query.filter(dsl::role_name.like(format!("%{}%", role_name)));
            }
            if let Some(description) = &f.description {
                query = query.filter(dsl::description.like(format!("%{}%", description)));
            }
            if let Some(permissions) = &f.permissions {
                query = query.filter(dsl::permissions.like(format!("%{}%", permissions)));
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
        }
        let data = query.load::<Role>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}
#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pg_connection, establish_pool};
    use obj_traits::request::pagination_request_param::PaginationParam;
    use obj_traits::request::request_param::RequestParam;

    use crate::models::role::{PatchRole, PostRole};
    use crate::models::role_filter::RoleFilter;

    use super::*;

    #[test]
    fn test_add_single() {
        let new_role = PostRole::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let result = RoleMapper::add_single(&mut conn, &new_role);
        assert!(result.is_ok());
        let inserted_role = result.unwrap();
        assert_eq!(inserted_role.role_name, new_role.role_name);
    }

    #[test]
    fn test_get_all() {
        let param = RequestParam::new(None, None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let result = RoleMapper::get_all(&mut conn, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.data().len() > 0);
    }

    #[test]
    fn test_get_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let result = RoleMapper::get_by_id(&mut conn, 2);
        assert!(result.is_ok());
        let role = result.unwrap();
        assert_eq!(role.role_id, 2);
    }

    #[test]
    fn test_delete_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_role = PostRole::new("DeleteUser".to_owned(), Some(String::new()), None);
        let inserted_role =
            RoleMapper::add_single(&mut conn, &new_role).expect("Failed to insert role");

        let delete_result = RoleMapper::delete_by_id(&mut conn, inserted_role.role_id);
        assert!(delete_result.is_ok());
        let deleted_role = delete_result.unwrap();
        assert_eq!(deleted_role.role_id, inserted_role.role_id);

        let get_result = RoleMapper::get_by_id(&mut conn, inserted_role.role_id);
        assert!(get_result.is_err());
    }

    #[test]
    fn test_update_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_role = PostRole::new("UpdatedUser".to_owned(), None, None);
        let inserted_role =
            RoleMapper::add_single(&mut conn, &new_role).expect("Failed to insert role");

        let updated_role = PatchRole {
            role_name: String::from("updated_role"),
            description: Some(String::from("Updated description")),
            permissions: Some(String::from("updated:permissions")),
            created_at: inserted_role.created_at,
            updated_at: Some(get_e8_time()),
        };

        let result = RoleMapper::update_by_id(&mut conn, inserted_role.role_id, &updated_role);
        assert!(result.is_ok());
        let updated_role_result = result.unwrap();
        assert_eq!(updated_role_result.role_name, updated_role.role_name);
    }

    #[test]
    fn test_filter() {
        let param = RequestParam {
            auth: None,
            pagination: Some(PaginationParam::default()),
            filter: Some(RoleFilter {
                role_name: Some("Admin".to_string()),
                role_id: None,
                description: None,
                permissions: None,
                created_at_min: None,
                created_at_max: None,
                updated_at_min: None,
                updated_at_max: None,
            }),
        };

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let result = RoleMapper::filter(&mut conn, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.data().len() > 0);
    }
}
