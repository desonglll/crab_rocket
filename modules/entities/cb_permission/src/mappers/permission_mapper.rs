use diesel::{prelude::*, result::Error};

use crab_rocket_schema::schema::permission_table::dsl;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};

use crate::models::{
    permission::{PatchPermission, Permission, PostPermission},
    permission_filter::PermissionFilter,
};

pub struct PermissionMapper {}

impl MapperCRUD for PermissionMapper {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Param = RequestParam<PermissionFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, Error> {
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
        let total_count = dsl::permission_table.count().get_result::<i64>(conn)? as i32;
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
        let data = dsl::permission_table
            .order(dsl::updated_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Permission>(conn)?;
        let body = Data::new(data, pagination);
        println!("{body}");
        Ok(body)
    }

    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Permission, Error> {
        // 配合 use crate::schema::posts::dsl::*;
        dsl::permission_table.filter(dsl::permission_id.eq(pid)).first(conn)
    }

    fn add_single(conn: &mut PgConnection, obj: &PostPermission) -> Result<Permission, Error> {
        match diesel::insert_into(dsl::permission_table)
            .values(obj)
            .returning(Permission::as_returning())
            .get_result(conn)
        {
            Ok(inserted_role) => Ok(inserted_role),
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
        }
    }

    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Permission, Error> {
        diesel::delete(dsl::permission_table.filter(dsl::permission_id.eq(pid))).get_result(conn)
    }

    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchPermission,
    ) -> Result<Permission, Error> {
        diesel::update(dsl::permission_table.filter(dsl::permission_id.eq(pid)))
            .set((
                dsl::permission_name.eq(obj.permission_name.clone()), // 確保 PatchPermission 中的方法與這裡的字段名稱匹配
                dsl::permission_description.eq(obj.permission_description.clone()),
                dsl::resource.eq(obj.resource.clone()),
                dsl::action.eq(obj.action.clone()),
                dsl::is_active.eq(obj.is_active),
                dsl::created_by.eq(obj.created_by.clone()),
                dsl::updated_by.eq(obj.updated_by.clone()),
                dsl::created_at.eq(obj.created_at),
                dsl::updated_at.eq(obj.updated_at),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, diesel::result::Error> {
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
        let total_count = dsl::permission_table.count().get_result::<i64>(conn)? as i32;
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

        let mut query = dsl::permission_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::created_at.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        let filter = &param.filter;
        if let Some(f) = filter {
            if let Some(permission_id) = f.permission_id {
                query = query.filter(dsl::permission_id.eq(permission_id));
            }
            if let Some(permission_name) = &f.permission_name {
                query = query.filter(dsl::permission_name.like(format!("%{}%", permission_name)));
            }
            if let Some(permission_description) = &f.permission_description {
                query = query.filter(
                    dsl::permission_description.like(format!("%{}%", permission_description)),
                );
            }
            if let Some(resource) = &f.resource {
                query = query.filter(dsl::resource.like(format!("%{}%", resource)));
            }
            if let Some(action) = &f.action {
                query = query.filter(dsl::action.like(format!("%{}%", action)));
            }
            if let Some(is_active) = f.is_active {
                query = query.filter(dsl::is_active.eq(is_active));
            }
            if let Some(created_by) = &f.created_by {
                query = query.filter(dsl::created_by.like(format!("%{}%", created_by)));
            }
            if let Some(updated_by) = &f.updated_by {
                query = query.filter(dsl::updated_by.like(format!("%{}%", updated_by)));
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
        let data = query.load::<Permission>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}
#[cfg(test)]
mod tests {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pg_connection, establish_pool};

    use super::*;

    #[test]
    fn test_get_all() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let param = RequestParam::demo();
        let result = PermissionMapper::get_all(&mut conn, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.data().len() > 0); // Ensure there's data or at least an empty vector
    }
    #[test]
    fn test_get_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_permission = PostPermission::demo();
        let permission = PermissionMapper::add_single(&mut conn, &new_permission).unwrap();
        let result = PermissionMapper::get_by_id(&mut conn, permission.permission_id);
        assert!(result.is_ok());
        let fetched_permission = result.unwrap();
        assert_eq!(fetched_permission.permission_id, permission.permission_id);
    }
    #[test]
    fn test_add_single() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_permission = PostPermission::demo();
        let result = PermissionMapper::add_single(&mut conn, &new_permission);
        assert!(result.is_ok());
        let inserted_permission = result.unwrap();
        assert_eq!(inserted_permission.permission_name, new_permission.permission_name);
    }
    #[test]
    fn test_delete_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_permission = PostPermission::demo();
        let inserted_permission = PermissionMapper::add_single(&mut conn, &new_permission).unwrap();
        let result = PermissionMapper::delete_by_id(&mut conn, inserted_permission.permission_id);
        assert!(result.is_ok());
        let deleted_permission = result.unwrap();
        assert_eq!(deleted_permission.permission_id, inserted_permission.permission_id);
    }
    #[test]
    fn test_update_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let new_permission = PostPermission::demo();
        let inserted_permission = PermissionMapper::add_single(&mut conn, &new_permission).unwrap();
        // Create a PatchPermission manually
        let updated_permission = PatchPermission {
            permission_name: "updated_name".to_string(),
            permission_description: inserted_permission.permission_description.clone(),
            resource: inserted_permission.resource.clone(),
            action: inserted_permission.action.clone(),
            is_active: inserted_permission.is_active,
            created_at: inserted_permission.created_at,
            updated_at: inserted_permission.updated_at,
            created_by: inserted_permission.created_by.clone(),
            updated_by: inserted_permission.updated_by.clone(),
            notes: inserted_permission.notes.clone(),
        };

        let result = PermissionMapper::update_by_id(
            &mut conn,
            inserted_permission.permission_id,
            &updated_permission,
        );
        assert!(result.is_ok());
        let updated_permission_result = result.unwrap();
        assert_eq!(updated_permission_result.permission_name, updated_permission.permission_name);
    }

    #[test]
    fn test_filter() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let mut conn = establish_pg_connection(pool).expect("Failed to establish connection");
        let param = RequestParam::demo();
        let result = PermissionMapper::filter(&mut conn, &param);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.data().len() >= 1); // Ensure at least one record matches the filter
    }
}
