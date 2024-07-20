use crate::models::{
    permission::{NewPermission, PatchPermission, Permission},
    permission_filter::PermissionFilter,
};
use crab_rocket_schema::schema::permission_table::dsl;
use diesel::{prelude::*, result::Error};
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

pub struct PermissionMapper {}

impl MapperCRUD for PermissionMapper {
    type Item = Permission;
    type NewItem = NewPermission;
    type PatchItem = PatchPermission;
    type Param = RequestParam<PaginationParam, PermissionFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, PermissionFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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

    fn add_single(conn: &mut PgConnection, obj: &NewPermission) -> Result<Permission, Error> {
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
        param: &RequestParam<PaginationParam, PermissionFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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
