use crate::models::employee::{Employee, NewEmployee, PatchEmployee};
use crate::models::info::employee_info::EmployeeInfo;
use crate::routes::models::employee_param::EmployeeParam;
use crate::schema::employee_table::{self, dsl::*};
use crate::utils::time::get_e8_time;
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

pub fn update_employee_by_id(
    conn: &mut PgConnection,
    id: i32,
    emp: &PatchEmployee,
) -> Result<Employee, diesel::result::Error> {
    diesel::update(employee_table.filter(employee_id.eq(id)))
        .set((
            employee_table::first_name.eq(emp.first_name.clone()),
            employee_table::last_name.eq(emp.last_name.clone()),
            employee_table::employee_name.eq(emp.employee_name.clone()),
            employee_table::gender.eq(emp.gender.clone()),
            employee_table::date_of_birth.eq(emp.date_of_birth.clone()),
            employee_table::hire_date.eq(emp.hire_date.clone()),
            employee_table::email.eq(emp.email.clone()),
            employee_table::phone_number.eq(emp.phone_number.clone()),
            employee_table::department_id.eq(emp.department_id.clone()),
            employee_table::job_title.eq(emp.job_title.clone()),
            employee_table::salary.eq(emp.salary.clone()),
            employee_table::manager_id.eq(emp.manager_id.clone()),
            employee_table::address.eq(emp.address.clone()),
            employee_table::city.eq(emp.city.clone()),
            employee_table::state.eq(emp.state.clone()),
            employee_table::postal_code.eq(emp.postal_code.clone()),
            employee_table::valid.eq(emp.valid.clone()),
            employee_table::last_update.eq(get_e8_time()),
        ))
        .get_result(conn)
}

pub fn fetch_employee_by_params(
    conn: &mut PgConnection,
    params: &EmployeeParam,
) -> (Result<Vec<Employee>, diesel::result::Error>, EmployeeInfo) {
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
    let filtered_emp = query
        .order(employee_table::employee_id.asc())
        .load::<Employee>(conn);
    let count: i64 = employee_table
        .count()
        .first(conn)
        .expect("Error counting employees");
    (filtered_emp, EmployeeInfo { count })
}

#[cfg(test)]
mod test {
    use crate::{
        establish_pg_connection, models::employee::NewEmployee,
        routes::models::employee_param::EmployeeParam,
    };

    use super::{delete_employee_by_id, fetch_employee_by_params, insert_employee};

    #[test]
    fn test_insert_employee() {
        let new_employee = NewEmployee::new_empty("mike");
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
            Ok(mut conn) => match fetch_employee_by_params(&mut conn, &params).0 {
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
        use crate::utils::time::get_e8_time;

        let updated_emp = crate::models::employee::PatchEmployee {
            employee_name: "John Doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            gender: Some("M".to_string()),
            date_of_birth: Some(get_e8_time()),
            hire_date: Some(get_e8_time()),
            email: Some("john.doe@example.com".to_string()),
            phone_number: Some("+1234567890".to_string()),
            department_id: Some(1),
            job_title: Some("Software Engineer".to_string()),
            salary: Some(1),
            manager_id: Some(6),
            address: Some("123 Main St".to_string()),
            city: Some("Anytown".to_string()),
            state: Some("Anystate".to_string()),
            postal_code: Some("12345".to_string()),
            valid: Some(true),
        };
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
