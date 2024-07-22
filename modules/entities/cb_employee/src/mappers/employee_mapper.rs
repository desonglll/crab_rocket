use crab_rocket_utils::time::get_e8_time;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::Pagination, request_param::RequestParam},
    response::data::Data,
};

use crate::models::{
    employee::{Employee, PatchEmployee, PostEmployee},
    employee_filter::EmployeeFilter,
};
use crab_rocket_schema::schema::employee_table::dsl;
use diesel::{prelude::*, result::Error};
pub struct EmployeeMapper {}

impl MapperCRUD for EmployeeMapper {
    type Item = Employee;
    type PostItem = PostEmployee;
    type PatchItem = PatchEmployee;
    type Param = RequestParam<EmployeeFilter>;
    fn get_all(
        conn: &mut PgConnection,
        param: &RequestParam<EmployeeFilter>,
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
        let pagination = param.pagination.as_ref().unwrap();
        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
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
    fn add_single(conn: &mut PgConnection, obj: &PostEmployee) -> Result<Employee, Error> {
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
                dsl::first_name.eq(&obj.first_name),
                dsl::last_name.eq(&obj.last_name),
                dsl::employee_name.eq(&obj.employee_name),
                dsl::gender.eq(&obj.gender),
                dsl::date_of_birth.eq(&obj.date_of_birth),
                dsl::hire_date.eq(&obj.hire_date),
                dsl::email.eq(&obj.email),
                dsl::phone_number.eq(&obj.phone_number),
                dsl::department_id.eq(&obj.department_id),
                dsl::job_title.eq(&obj.job_title),
                dsl::salary.eq(&obj.salary),
                dsl::manager_id.eq(&obj.manager_id),
                dsl::address.eq(&obj.address),
                dsl::city.eq(&obj.city),
                dsl::state.eq(&obj.state),
                dsl::postal_code.eq(&obj.postal_code),
                dsl::valid.eq(&obj.valid),
                dsl::last_update.eq(get_e8_time()),
                dsl::role_name.eq(&obj.role_name),
                dsl::role_id.eq(obj.role_id),
            ))
            .get_result(conn)
    }
    fn filter(
        conn: &mut PgConnection,
        param: &RequestParam<EmployeeFilter>,
    ) -> Result<Data<Vec<Employee>>, diesel::result::Error> {
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
        let pagination = param.pagination.as_ref().unwrap();
        let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
        let per_page = pagination.limit.unwrap();
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

        let mut query = dsl::employee_table.into_boxed();

        // 分页查询
        query = query
            .order(dsl::last_update.desc())
            .limit(per_page as i64)
            .offset(((page - 1) * per_page) as i64);

        if let Some(f) = filter {
            if let Some(first_name) = &f.first_name {
                query = query.filter(dsl::first_name.like(format!("%{}%", first_name)));
            }
            if let Some(last_name) = &f.last_name {
                query = query.filter(dsl::last_name.like(format!("%{}%", last_name)));
            }
            if let Some(gender) = &f.gender {
                query = query.filter(dsl::gender.eq(gender));
            }
            if let Some(date_of_birth_min) = &f.date_of_birth_min {
                query = query.filter(dsl::date_of_birth.ge(date_of_birth_min));
            }
            if let Some(date_of_birth_max) = &f.date_of_birth_max {
                query = query.filter(dsl::date_of_birth.le(date_of_birth_max));
            }
            if let Some(hire_date_min) = &f.hire_date_min {
                query = query.filter(dsl::hire_date.ge(hire_date_min));
            }
            if let Some(hire_date_max) = &f.hire_date_max {
                query = query.filter(dsl::hire_date.le(hire_date_max));
            }
            if let Some(department_id) = &f.department_id {
                query = query.filter(dsl::department_id.eq(department_id));
            }
            if let Some(job_title) = &f.job_title {
                query = query.filter(dsl::job_title.like(format!("%{}%", job_title)));
            }
            if let Some(salary_min) = &f.salary_min {
                query = query.filter(dsl::salary.ge(salary_min));
            }
            if let Some(salary_max) = &f.salary_max {
                query = query.filter(dsl::salary.le(salary_max));
            }
            if let Some(manager_id) = &f.manager_id {
                query = query.filter(dsl::manager_id.eq(manager_id));
            }
            if let Some(address) = &f.address {
                query = query.filter(dsl::address.like(format!("%{}%", address)));
            }
            if let Some(city) = &f.city {
                query = query.filter(dsl::city.like(format!("%{}%", city)));
            }
            if let Some(state) = &f.state {
                query = query.filter(dsl::state.like(format!("%{}%", state)));
            }
            if let Some(postal_code) = &f.postal_code {
                query = query.filter(dsl::postal_code.like(format!("%{}%", postal_code)));
            }
            if let Some(valid) = &f.valid {
                query = query.filter(dsl::valid.eq(valid));
            }
            if let Some(last_update_min) = &f.last_update_min {
                query = query.filter(dsl::last_update.ge(last_update_min));
            }
            if let Some(last_update_max) = &f.last_update_max {
                query = query.filter(dsl::last_update.le(last_update_max));
            }
            if let Some(role_name) = &f.role_name {
                query = query.filter(dsl::role_name.like(format!("%{}%", role_name)));
            }
            if let Some(role_id) = &f.role_id {
                query = query.filter(dsl::role_id.eq(role_id));
            }
        }
        let data = query.load::<Employee>(conn)?;
        let body = Data::new(data, pagination);
        Ok(body)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::models::{
        employee::{PatchEmployee, PostEmployee},
        employee_filter::EmployeeFilter,
    };
    use crab_rocket_schema::{establish_pg_connection, establish_pool, DbPool};
    use obj_traits::{mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam};
    use rocket::State;

    #[test]
    fn test_insert_employee() {
        let new_employee = PostEmployee {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            employee_name: "John Doe".to_string(),
            gender: Some("Male".to_string()),
            date_of_birth: Some(get_e8_time()),
            hire_date: Some(get_e8_time()),
            email: Some("john.doe@example.com".to_string()),
            phone_number: Some("1234567890".to_string()),
            department_id: Some(1),
            job_title: Some("Developer".to_string()),
            salary: Some(60000.0),
            manager_id: Some(2),
            address: Some("123 Main St".to_string()),
            city: Some("Metropolis".to_string()),
            state: Some("NY".to_string()),
            postal_code: Some("12345".to_string()),
            valid: Some(true),
            role_name: Some("Senior Developer".to_string()),
            role_id: Some(1),
        };

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => match EmployeeMapper::add_single(&mut conn, &new_employee) {
                Ok(inserted_employee) => println!("{:?}", inserted_employee),
                Err(e) => eprintln!("Error inserting employee: {:?}", e),
            },
            Err(e) => eprintln!("Error establishing connection: {:?}", e),
        }
    }

    #[test]
    fn test_delete_employee_by_id() {
        let employee_id = 1; // Ensure this ID exists in your test database

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => match EmployeeMapper::delete_by_id(&mut conn, employee_id) {
                Ok(deleted_employee) => println!("{:?}", deleted_employee),
                Err(e) => eprintln!("Error deleting employee: {:?}", e),
            },
            Err(e) => eprintln!("Error establishing connection: {:?}", e),
        }
    }

    #[test]
    fn test_fetch_employee_by_params() {
        let json_data = r#"
        {
            "gender": "Male"
        }
        "#;
        let filter = EmployeeFilter::from_json(json_data).unwrap();
        let params = RequestParam::new(None, Some(filter));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => match EmployeeMapper::get_all(&mut conn, &params) {
                Ok(result) => println!("{:?}", result),
                Err(e) => eprintln!("Error fetching employees: {:?}", e),
            },
            Err(e) => eprintln!("Error establishing connection: {:?}", e),
        }
    }

    #[test]
    fn test_update_employee_by_id() {
        let updated_emp = PatchEmployee {
            first_name: Some("Jane".to_string()),
            last_name: Some("Doe".to_string()),
            employee_name: "Jane Doe".to_string(),
            gender: Some("Female".to_string()),
            date_of_birth: Some(get_e8_time()),
            hire_date: Some(get_e8_time()),
            email: Some("jane.doe@example.com".to_string()),
            phone_number: Some("0987654321".to_string()),
            department_id: Some(2),
            job_title: Some("Lead Developer".to_string()),
            salary: Some(80000.0),
            manager_id: Some(3),
            address: Some("456 Another St".to_string()),
            city: Some("Gotham".to_string()),
            state: Some("CA".to_string()),
            postal_code: Some("67890".to_string()),
            valid: Some(false),
            role_name: Some("Lead Developer".to_string()),
            role_id: Some(2),
        };

        let employee_id = 2; // Ensure this ID exists in your test database

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => {
                match EmployeeMapper::update_by_id(&mut conn, employee_id, &updated_emp) {
                    Ok(updated_employee) => println!("{:?}", updated_employee),
                    Err(e) => eprintln!("Error updating employee: {:?}", e),
                }
            }
            Err(e) => eprintln!("Error establishing connection: {:?}", e),
        }
    }

    #[test]
    fn test_filter_employees() {
        let json_data = r#"
        {
            "salary_min": 50000.0,
            "salary_max": 70000.0
        }
        "#;
        let filter = EmployeeFilter::from_json(json_data).unwrap();
        let params = RequestParam::new(None, Some(filter));

        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => match EmployeeMapper::filter(&mut conn, &params) {
                Ok(result) => println!("{:?}", result),
                Err(e) => eprintln!("Error filtering employees: {:?}", e),
            },
            Err(e) => eprintln!("Error establishing connection: {:?}", e),
        }
    }
}
