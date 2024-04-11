use crate::models::employee::{Employee, NewEmployee};
use crate::schema::employee_table::{self, dsl::*};
use diesel::prelude::*;

pub fn insert_employee(
    conn: &mut PgConnection,
    new_employee: &NewEmployee,
) -> Result<Employee, diesel::result::Error> {
    let new_employee = diesel::insert_into(employee_table)
        .values(new_employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .expect("Error saving new employee");
    Ok(new_employee)
}
// FIXME: a bug for Queryable
// pub fn delete_employee_by_id(
//     conn: &mut PgConnection,
//     id: i32,
// ) -> Result<Employee, diesel::result::Error> {
//     diesel::delete(employee_table.filter(employee_table::employee_id.
// eq(id))).get_result(conn) }

pub fn delete_employee_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(employee_table.filter(employee_table::employee_id.eq(id))).execute(conn)
}

#[cfg(test)]
mod test {
    use crate::{establish_pg_connection, models::employee::NewEmployee};

    use super::insert_employee;

    #[test]
    fn test_insert_employee() {
        let new_employee = NewEmployee::new_empty("root");
        match establish_pg_connection() {
            Ok(mut conn) => match insert_employee(&mut conn, &new_employee) {
                Ok(inserted_employee) => println!("{inserted_employee:?}"),
                Err(_) => println!("error"),
            },
            Err(_) => println!("Error"),
        }
    }
}
