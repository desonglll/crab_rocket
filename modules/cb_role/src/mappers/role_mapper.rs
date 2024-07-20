use crate::models::role::{PostRole, PatchRole, Role};
use crate::models::role_filter::RoleFilter;
use crab_rocket_schema::schema::role_table::dsl; //配合下面的 `posts.filter()`
use crab_rocket_schema::schema::role_table::{self};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::{Pagination, PaginationParam};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;

pub struct RoleMapper {}

impl MapperCRUD for RoleMapper {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Param = RequestParam<PaginationParam, RoleFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, RoleFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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
                role_table::role_name.eq(obj.role_name()),
                role_table::description.eq(obj.description()),
                role_table::permissions.eq(obj.permissions()),
                role_table::created_at.eq(obj.created_at()),
                role_table::updated_at.eq(get_e8_time()),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam, RoleFilter>,
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
        let page = (param.pagination.offset.unwrap() / param.pagination.limit.unwrap()) + 1;
        let per_page = param.pagination.limit.unwrap();
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
mod test {
    use obj_traits::request::pagination_request_param::PaginationParamTrait;

    #[test]
    fn test_insert_role() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                // 创建一个新的 NewPost 实例
                let new_role = PostRole::demo();
                let _ = RoleMapper::add_single(&mut conn, &new_role);
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_all_roles() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        let param = RequestParam::new(PaginationParam::demo(), None);

        match establish_pg_connection() {
            Ok(mut conn) => match RoleMapper::get_all(&mut conn, &param) {
                Ok(roles) => {
                    println!("{roles}");
                    ()
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }
}
