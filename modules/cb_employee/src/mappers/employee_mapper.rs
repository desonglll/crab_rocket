use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::routes::employee_param::EmployeeParam;
use crab_rocket_schema::schema::employee_table::{self, dsl::*};
use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;

pub fn insert_employee(
    conn: &mut PgConnection,
    new_employee: &NewEmployee,
) -> Result<Employee, diesel::result::Error> {
    let new_employee = diesel::insert_into(employee_table)
        .values(new_employee)
        .returning(Employee::as_returning())
        .get_result(conn);
    match new_employee {
        Ok(new_employee) => Ok(new_employee),
        Err(e) => {
            println!("{e:?}");
            Err(e)
        }
    }
}

// GOOD: a bug for Queryable
// Found Reason: The sequence is incorent.
pub fn delete_employee_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<Employee, diesel::result::Error> {
    let result =
        diesel::delete(employee_table.filter(employee_table::employee_id.eq(id))).get_result(conn);
    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("{e:?}");
            Err(e)
        }
    }
}
// GOOD:
pub fn fetch_employee_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<Employee, diesel::result::Error> {
    employee_table.filter(employee_table::employee_id.eq(id)).first(conn)
}
pub fn update_employee_by_id(
    conn: &mut PgConnection,
    id: i32,
    emp: &PatchEmployee,
) -> Result<Employee, diesel::result::Error> {
    diesel::update(employee_table.filter(employee_id.eq(id)))
        .set((
            employee_table::first_name.eq(emp.first_name()),
            employee_table::last_name.eq(emp.last_name()),
            employee_table::employee_name.eq(emp.employee_name()),
            employee_table::gender.eq(emp.gender()),
            employee_table::date_of_birth.eq(emp.date_of_birth()),
            employee_table::hire_date.eq(emp.hire_date()),
            employee_table::email.eq(emp.email()),
            employee_table::phone_number.eq(emp.phone_number()),
            employee_table::department_id.eq(emp.department_id()),
            employee_table::job_title.eq(emp.job_title()),
            employee_table::salary.eq(emp.salary()),
            employee_table::manager_id.eq(emp.manager_id()),
            employee_table::address.eq(emp.address()),
            employee_table::city.eq(emp.city()),
            employee_table::state.eq(emp.state()),
            employee_table::postal_code.eq(emp.postal_code()),
            employee_table::valid.eq(emp.valid()),
            employee_table::last_update.eq(get_e8_time()),
            employee_table::role_name.eq(emp.role_name()),
            employee_table::role_id.eq(emp.role_id()),
        ))
        .get_result(conn)
}

pub fn fetch_employee_by_params(
    conn: &mut PgConnection,
    params: &EmployeeParam,
) -> Result<Vec<Employee>, diesel::result::Error> {
    let mut query = employee_table.into_boxed();

    if let Some(emp_id) = params.employee_id {
        if emp_id != 0 {
            query = query.filter(employee_table::employee_id.eq(emp_id));
        }
    }
    if let Some(limit) = params.limit {
        if limit != 0 {
            query = query.limit(limit.into());
        }
    }
    if let Some(offset) = params.offset {
        if offset != 0 {
            query = query.offset(offset.into());
        }
    }
    let filtered_emp = query.order(employee_table::employee_id.asc()).load::<Employee>(conn);

    filtered_emp
}

#[cfg(test)]
mod test {
    use crate::{models::employee::NewEmployee, routes::employee_param::EmployeeParam};
    use crab_rocket_schema::establish_pg_connection;

    use super::{delete_employee_by_id, fetch_employee_by_params, insert_employee};

    #[test]
    fn test_insert_employee() {
        let new_employee = NewEmployee::default();
        match establish_pg_connection() {
            Ok(mut conn) => match insert_employee(&mut conn, &new_employee) {
                Ok(inserted_employee) => println!("{inserted_employee:?}"),
                Err(_) => println!("error"),
            },
            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn test_delete_employee_by_id() {
        match establish_pg_connection() {
            Ok(mut conn) => match delete_employee_by_id(&mut conn, 1) {
                Ok(deleted_employee) => println!("{deleted_employee:?}"),
                Err(_) => println!("error"),
            },
            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn test_fetch_employee_by_params() {
        let params = EmployeeParam {
            employee_id: Some(5),
            limit: None,
            offset: None,
        };
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_employee_by_params(&mut conn, &params) {
                Ok(u_posts) => {
                    println!("{u_posts:?}")
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }

    #[test]
    fn test_update_employee_by_id() {
        use super::*;
        use crab_rocket_utils::time::get_e8_time;

        let updated_emp = crate::models::employee::PatchEmployee::new(
            "John Doe".to_string(),
            Some("John".to_string()),
            Some("Doe".to_string()),
            Some("M".to_string()),
            Some(get_e8_time()),
            Some(get_e8_time()),
            Some("john.doe@example.com".to_string()),
            Some("+1234567890".to_string()),
            Some(1),
            Some("Software Engineer".to_string()),
            Some(1),
            Some(6),
            Some("123 Main St".to_string()),
            Some("Anytown".to_string()),
            Some("Anystate".to_string()),
            Some("12345".to_string()),
            Some(true),
            Some("admin".to_string()),
            Some(1),
        );
        match establish_pg_connection() {
            Ok(mut conn) => match update_employee_by_id(&mut conn, 5, &updated_emp) {
                Ok(updated_emp) => {
                    println!("{updated_emp:?}")
                }
                Err(e) => {
                    println!("{e:?}");
                    ()
                }
            },
            Err(_) => (),
        }
    }
}
