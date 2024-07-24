use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Selectable, Insertable, Debug, Serialize, Deserialize, Queryable, ToSchema, Default, Clone,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub employee_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub employee_name: String,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub hire_date: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
    pub last_update: Option<chrono::NaiveDateTime>,
    pub role_name: Option<String>,
    pub role_id: Option<i32>,
}

impl Employee {
    pub fn new(
        employee_id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        employee_name: String,
        gender: Option<String>,
        date_of_birth: Option<chrono::NaiveDateTime>,
        hire_date: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        phone_number: Option<String>,
        department_id: Option<i32>,
        job_title: Option<String>,
        salary: Option<f64>,
        manager_id: Option<i32>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        valid: Option<bool>,
        last_update: Option<chrono::NaiveDateTime>,
        role_name: Option<String>,
        role_id: Option<i32>,
    ) -> Self {
        Self {
            employee_id,
            first_name,
            last_name,
            employee_name,
            gender,
            date_of_birth,
            hire_date,
            email,
            phone_number,
            department_id,
            job_title,
            salary,
            manager_id,
            address,
            city,
            state,
            postal_code,
            valid,
            last_update,
            role_name,
            role_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostEmployee {
    pub employee_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub hire_date: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
    pub role_name: Option<String>,
    pub role_id: Option<i32>,
}

impl PostEmployee {
    pub fn new(
        employee_name: String,
        first_name: Option<String>,
        last_name: Option<String>,
        gender: Option<String>,
        date_of_birth: Option<chrono::NaiveDateTime>,
        hire_date: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        phone_number: Option<String>,
        department_id: Option<i32>,
        job_title: Option<String>,
        salary: Option<f64>,
        manager_id: Option<i32>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        valid: Option<bool>,
        role_name: Option<String>,
        role_id: Option<i32>,
    ) -> Self {
        Self {
            employee_name,
            first_name,
            last_name,
            gender,
            date_of_birth,
            hire_date,
            email,
            phone_number,
            department_id,
            job_title,
            salary,
            manager_id,
            address,
            city,
            state,
            postal_code,
            valid,
            role_name,
            role_id,
        }
    }
    pub fn demo() -> Self {
        Self {
            employee_name: "JohnDoe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            gender: Some("Male".to_string()),
            date_of_birth: Some(get_e8_time()), // Example: 1990-01-01 00:00:00
            hire_date: Some(get_e8_time()),     // Example: 2020-01-01 00:00:00
            email: Some("johndoe@example.com".to_string()),
            phone_number: Some("123-456-7890".to_string()),
            department_id: Some(1),
            job_title: Some("Software Engineer".to_string()),
            salary: Some(60000.0),
            manager_id: Some(2),
            address: Some("123 Main St".to_string()),
            city: Some("Anytown".to_string()),
            state: Some("Anystate".to_string()),
            postal_code: Some("12345".to_string()),
            valid: Some(true),
            role_name: Some("Developer".to_string()),
            role_id: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchEmployee {
    pub employee_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub hire_date: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
    pub role_name: Option<String>,
    pub role_id: Option<i32>,
}

impl PatchEmployee {
    pub fn new(
        employee_name: String,
        first_name: Option<String>,
        last_name: Option<String>,
        gender: Option<String>,
        date_of_birth: Option<chrono::NaiveDateTime>,
        hire_date: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        phone_number: Option<String>,
        department_id: Option<i32>,
        job_title: Option<String>,
        salary: Option<f64>,
        manager_id: Option<i32>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        valid: Option<bool>,
        role_name: Option<String>,
        role_id: Option<i32>,
    ) -> Self {
        Self {
            employee_name,
            first_name,
            last_name,
            gender,
            date_of_birth,
            hire_date,
            email,
            phone_number,
            department_id,
            job_title,
            salary,
            manager_id,
            address,
            city,
            state,
            postal_code,
            valid,
            role_name,
            role_id,
        }
    }

    pub fn demo() -> Self {
        Self {
            employee_name: "JohnDoe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            gender: Some("Male".to_string()),
            date_of_birth: Some(get_e8_time()), // Example: 1990-01-01 00:00:00
            hire_date: Some(get_e8_time()),     // Example: 2020-01-01 00:00:00
            email: Some("johndoe@example.com".to_string()),
            phone_number: Some("123-456-7890".to_string()),
            department_id: Some(1),
            job_title: Some("Software Engineer".to_string()),
            salary: Some(60000.0),
            manager_id: Some(2),
            address: Some("123 Main St".to_string()),
            city: Some("Anytown".to_string()),
            state: Some("Anystate".to_string()),
            postal_code: Some("12345".to_string()),
            valid: Some(true),
            role_name: Some("Developer".to_string()),
            role_id: Some(1),
        }
    }
}
impl From<Employee> for PostEmployee {
    fn from(employee: Employee) -> Self {
        PostEmployee {
            employee_name: employee.employee_name,
            first_name: employee.first_name,
            last_name: employee.last_name,
            gender: employee.gender,
            date_of_birth: employee.date_of_birth,
            hire_date: employee.hire_date,
            email: employee.email,
            phone_number: employee.phone_number,
            department_id: employee.department_id,
            job_title: employee.job_title,
            salary: employee.salary,
            manager_id: employee.manager_id,
            address: employee.address,
            city: employee.city,
            state: employee.state,
            postal_code: employee.postal_code,
            valid: employee.valid,
            role_name: employee.role_name,
            role_id: employee.role_id,
        }
    }
}

impl From<Employee> for PatchEmployee {
    fn from(employee: Employee) -> Self {
        PatchEmployee {
            employee_name: employee.employee_name,
            first_name: employee.first_name,
            last_name: employee.last_name,
            gender: employee.gender,
            date_of_birth: employee.date_of_birth,
            hire_date: employee.hire_date,
            email: employee.email,
            phone_number: employee.phone_number,
            department_id: employee.department_id,
            job_title: employee.job_title,
            salary: employee.salary,
            manager_id: employee.manager_id,
            address: employee.address,
            city: employee.city,
            state: employee.state,
            postal_code: employee.postal_code,
            valid: employee.valid,
            role_name: employee.role_name,
            role_id: employee.role_id,
        }
    }
}
