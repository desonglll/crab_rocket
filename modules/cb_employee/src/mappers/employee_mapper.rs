use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{
        pagination_request_param::{Pagination, PaginationParam},
        request_param::RequestParam,
    },
    response::data::Data,
};

use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crab_rocket_schema::schema::employee_table::dsl;
use diesel::{prelude::*, result::Error};
pub struct EmployeeMapper {}

impl MapperCRUD<Employee, NewEmployee, PatchEmployee, RequestParam<PaginationParam>>
    for EmployeeMapper
{
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<PaginationParam>,
    ) -> Result<obj_traits::response::data::Data<Vec<Employee>>, diesel::result::Error> {
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
        let total_count = dsl::employee_table.count().get_result::<i64>(conn)? as i32;
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
        let data = dsl::employee_table
            .order(dsl::last_update.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64)
            .load::<Employee>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Employee, Error> {
        dsl::employee_table.filter(dsl::employee_id.eq(pid)).first(conn)
    }
    fn add_single(conn: &mut PgConnection, obj: &NewEmployee) -> Result<Employee, Error> {
        diesel::insert_into(dsl::employee_table)
            .values(obj)
            .returning(Employee::as_returning())
            .get_result(conn)
    }
    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<Employee, Error> {
        diesel::delete(dsl::employee_table.filter(dsl::employee_id.eq(pid))).get_result(conn)
    }
    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &PatchEmployee,
    ) -> Result<Employee, Error> {
        diesel::update(dsl::employee_table.filter(dsl::employee_id.eq(pid)))
            .set((
                dsl::first_name.eq(obj.first_name()),
                dsl::last_name.eq(obj.last_name()),
                dsl::employee_name.eq(obj.employee_name()),
                dsl::gender.eq(obj.gender()),
                dsl::date_of_birth.eq(obj.date_of_birth()),
                dsl::hire_date.eq(obj.hire_date()),
                dsl::email.eq(obj.email()),
                dsl::phone_number.eq(obj.phone_number()),
                dsl::department_id.eq(obj.department_id()),
                dsl::job_title.eq(obj.job_title()),
                dsl::salary.eq(obj.salary()),
                dsl::manager_id.eq(obj.manager_id()),
                dsl::address.eq(obj.address()),
                dsl::city.eq(obj.city()),
                dsl::state.eq(obj.state()),
                dsl::postal_code.eq(obj.postal_code()),
                dsl::valid.eq(obj.valid()),
                dsl::last_update.eq(get_e8_time()),
                dsl::role_name.eq(obj.role_name()),
                dsl::role_id.eq(obj.role_id()),
            ))
            .get_result(conn)
    }
}
#[cfg(test)]
mod test {
    use crate::models::employee::{NewEmployee, PatchEmployee};
    use crab_rocket_schema::establish_pg_connection;
    use obj_traits::{
        mapper::mapper_crud::MapperCRUD,
        request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    };

    use super::EmployeeMapper;

    #[test]
    fn test_insert_employee() {
        let new_employee = NewEmployee::default();
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::add_single(&mut conn, &new_employee) {
                Ok(inserted_employee) => println!("{inserted_employee:?}"),
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_employee_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::delete_by_id(&mut conn, 1) {
                Ok(deleted_employee) => println!("{deleted_employee:?}"),
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_fetch_employee_by_params() {
        let params = RequestParam::new(PaginationParam::demo());
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::get_all(&mut conn, &params) {
                Ok(u_posts) => {
                    println!("{u_posts}")
                }
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_update_employee_by_id() {
        let updated_emp = PatchEmployee::demo();
        match establish_pg_connection() {
            Ok(mut conn) => match EmployeeMapper::update_by_id(&mut conn, 2, &updated_emp) {
                Ok(updated_emp) => {
                    println!("{updated_emp:?}")
                }
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }
}
