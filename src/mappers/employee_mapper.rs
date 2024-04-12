use crate::models::employee::{Employee, NewEmployee};
use crate::models::info::employee_info::EmployeeInfo;
use crate::routes::models::employee_param::EmployeeParam;
use crate::schema::employee_table::{self, dsl::*};
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
}
