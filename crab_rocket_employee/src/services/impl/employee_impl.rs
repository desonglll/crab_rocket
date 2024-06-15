use crate::{
    mappers::employee_mapper::{
        self, delete_employee_by_id, insert_employee, update_employee_by_id,
    },
    models::employee::{Employee, NewEmployee},
    services::employee_service::GetEmployee,
};
use crab_rocket_schema::establish_pg_connection;

impl GetEmployee for Employee {
    fn insert_employee(new_employee: &NewEmployee) -> Result<Employee, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match insert_employee(&mut conn, &new_employee) {
                    Ok(inserted_employee) => Ok(inserted_employee),
                    Err(e) => {
                        println!("error");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn delete_employee_by_id(id: i32) -> Result<Employee, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match delete_employee_by_id(&mut conn, id) {
                    Ok(deleted_employee) => Ok(deleted_employee),
                    Err(e) => {
                        println!("error");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn filter_employee_by_params(
        params: &crate::routes::employee_param::EmployeeParam,
    ) -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match employee_mapper::fetch_employee_by_params(&mut conn, params) {
                    Ok(filtered_emp) => Ok(filtered_emp),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn update_employee_by_id(
        id: i32,
        emp: &crate::models::employee::PatchEmployee,
    ) -> Result<Employee, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match update_employee_by_id(&mut conn, id, emp) {
                    Ok(updated_emp) => Ok(updated_emp),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}
