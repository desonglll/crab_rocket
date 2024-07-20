use crab_rocket_utils::time::get_e8_time;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Selectable, Insertable, Debug, Serialize, Deserialize, Queryable, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
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
    salary: Option<i32>,
    manager_id: Option<i32>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    valid: Option<bool>,
    last_update: Option<chrono::NaiveDateTime>,
    role_name: Option<String>,
    role_id: Option<i32>,
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
        salary: Option<i32>,
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

    pub fn employee_id(&self) -> i32 {
        self.employee_id
    }

    pub fn first_name(&self) -> &Option<String> {
        &self.first_name
    }

    pub fn last_name(&self) -> &Option<String> {
        &self.last_name
    }

    pub fn employee_name(&self) -> &str {
        &self.employee_name
    }

    pub fn gender(&self) -> &Option<String> {
        &self.gender
    }

    pub fn date_of_birth(&self) -> Option<chrono::NaiveDateTime> {
        self.date_of_birth
    }

    pub fn hire_date(&self) -> Option<chrono::NaiveDateTime> {
        self.hire_date
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn phone_number(&self) -> &Option<String> {
        &self.phone_number
    }

    pub fn department_id(&self) -> Option<i32> {
        self.department_id
    }

    pub fn job_title(&self) -> &Option<String> {
        &self.job_title
    }

    pub fn salary(&self) -> Option<i32> {
        self.salary
    }

    pub fn manager_id(&self) -> Option<i32> {
        self.manager_id
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }

    pub fn city(&self) -> &Option<String> {
        &self.city
    }

    pub fn state(&self) -> &Option<String> {
        &self.state
    }

    pub fn postal_code(&self) -> &Option<String> {
        &self.postal_code
    }

    pub fn valid(&self) -> Option<bool> {
        self.valid
    }

    pub fn last_update(&self) -> Option<chrono::NaiveDateTime> {
        self.last_update
    }

    pub fn role_name(&self) -> &Option<String> {
        &self.role_name
    }

    pub fn role_id(&self) -> Option<i32> {
        self.role_id
    }

    pub fn set_employee_id(&mut self, employee_id: i32) {
        self.employee_id = employee_id;
    }

    pub fn set_first_name(&mut self, first_name: Option<String>) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_employee_name(&mut self, employee_name: String) {
        self.employee_name = employee_name;
    }

    pub fn set_gender(&mut self, gender: Option<String>) {
        self.gender = gender;
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: Option<chrono::NaiveDateTime>) {
        self.date_of_birth = date_of_birth;
    }

    pub fn set_hire_date(&mut self, hire_date: Option<chrono::NaiveDateTime>) {
        self.hire_date = hire_date;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_department_id(&mut self, department_id: Option<i32>) {
        self.department_id = department_id;
    }

    pub fn set_job_title(&mut self, job_title: Option<String>) {
        self.job_title = job_title;
    }

    pub fn set_salary(&mut self, salary: Option<i32>) {
        self.salary = salary;
    }

    pub fn set_manager_id(&mut self, manager_id: Option<i32>) {
        self.manager_id = manager_id;
    }

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn set_city(&mut self, city: Option<String>) {
        self.city = city;
    }

    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    pub fn set_postal_code(&mut self, postal_code: Option<String>) {
        self.postal_code = postal_code;
    }

    pub fn set_valid(&mut self, valid: Option<bool>) {
        self.valid = valid;
    }

    pub fn set_last_update(&mut self, last_update: Option<chrono::NaiveDateTime>) {
        self.last_update = last_update;
    }

    pub fn set_role_name(&mut self, role_name: Option<String>) {
        self.role_name = role_name;
    }

    pub fn set_role_id(&mut self, role_id: Option<i32>) {
        self.role_id = role_id;
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmployee {
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
    salary: Option<i32>,
    manager_id: Option<i32>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    valid: Option<bool>,
    role_name: Option<String>,
    role_id: Option<i32>,
}

impl NewEmployee {
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
        salary: Option<i32>,
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
            salary: Some(60000),
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
    pub fn employee_name(&self) -> &str {
        &self.employee_name
    }

    pub fn first_name(&self) -> &Option<String> {
        &self.first_name
    }

    pub fn last_name(&self) -> &Option<String> {
        &self.last_name
    }

    pub fn gender(&self) -> &Option<String> {
        &self.gender
    }

    pub fn date_of_birth(&self) -> Option<chrono::NaiveDateTime> {
        self.date_of_birth
    }

    pub fn hire_date(&self) -> Option<chrono::NaiveDateTime> {
        self.hire_date
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn phone_number(&self) -> &Option<String> {
        &self.phone_number
    }

    pub fn department_id(&self) -> Option<i32> {
        self.department_id
    }

    pub fn job_title(&self) -> &Option<String> {
        &self.job_title
    }

    pub fn salary(&self) -> Option<i32> {
        self.salary
    }

    pub fn manager_id(&self) -> Option<i32> {
        self.manager_id
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }

    pub fn city(&self) -> &Option<String> {
        &self.city
    }

    pub fn state(&self) -> &Option<String> {
        &self.state
    }

    pub fn postal_code(&self) -> &Option<String> {
        &self.postal_code
    }

    pub fn valid(&self) -> Option<bool> {
        self.valid
    }

    pub fn role_name(&self) -> &Option<String> {
        &self.role_name
    }

    pub fn role_id(&self) -> Option<i32> {
        self.role_id
    }

    pub fn set_employee_name(&mut self, employee_name: String) {
        self.employee_name = employee_name;
    }

    pub fn set_first_name(&mut self, first_name: Option<String>) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_gender(&mut self, gender: Option<String>) {
        self.gender = gender;
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: Option<chrono::NaiveDateTime>) {
        self.date_of_birth = date_of_birth;
    }

    pub fn set_hire_date(&mut self, hire_date: Option<chrono::NaiveDateTime>) {
        self.hire_date = hire_date;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_department_id(&mut self, department_id: Option<i32>) {
        self.department_id = department_id;
    }

    pub fn set_job_title(&mut self, job_title: Option<String>) {
        self.job_title = job_title;
    }

    pub fn set_salary(&mut self, salary: Option<i32>) {
        self.salary = salary;
    }

    pub fn set_manager_id(&mut self, manager_id: Option<i32>) {
        self.manager_id = manager_id;
    }

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn set_city(&mut self, city: Option<String>) {
        self.city = city;
    }

    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    pub fn set_postal_code(&mut self, postal_code: Option<String>) {
        self.postal_code = postal_code;
    }

    pub fn set_valid(&mut self, valid: Option<bool>) {
        self.valid = valid;
    }

    pub fn set_role_name(&mut self, role_name: Option<String>) {
        self.role_name = role_name;
    }

    pub fn set_role_id(&mut self, role_id: Option<i32>) {
        self.role_id = role_id;
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchEmployee {
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
    salary: Option<i32>,
    manager_id: Option<i32>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    valid: Option<bool>,
    role_name: Option<String>,
    role_id: Option<i32>,
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
        salary: Option<i32>,
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
            salary: Some(60000),
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

    pub fn employee_name(&self) -> &str {
        &self.employee_name
    }

    pub fn first_name(&self) -> &Option<String> {
        &self.first_name
    }

    pub fn last_name(&self) -> &Option<String> {
        &self.last_name
    }

    pub fn gender(&self) -> &Option<String> {
        &self.gender
    }

    pub fn date_of_birth(&self) -> Option<chrono::NaiveDateTime> {
        self.date_of_birth
    }

    pub fn hire_date(&self) -> Option<chrono::NaiveDateTime> {
        self.hire_date
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn phone_number(&self) -> &Option<String> {
        &self.phone_number
    }

    pub fn department_id(&self) -> Option<i32> {
        self.department_id
    }

    pub fn job_title(&self) -> &Option<String> {
        &self.job_title
    }

    pub fn salary(&self) -> Option<i32> {
        self.salary
    }

    pub fn manager_id(&self) -> Option<i32> {
        self.manager_id
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }

    pub fn city(&self) -> &Option<String> {
        &self.city
    }

    pub fn state(&self) -> &Option<String> {
        &self.state
    }

    pub fn postal_code(&self) -> &Option<String> {
        &self.postal_code
    }

    pub fn valid(&self) -> Option<bool> {
        self.valid
    }

    pub fn role_name(&self) -> &Option<String> {
        &self.role_name
    }

    pub fn role_id(&self) -> Option<i32> {
        self.role_id
    }

    pub fn set_employee_name(&mut self, employee_name: String) {
        self.employee_name = employee_name;
    }

    pub fn set_first_name(&mut self, first_name: Option<String>) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: Option<String>) {
        self.last_name = last_name;
    }

    pub fn set_gender(&mut self, gender: Option<String>) {
        self.gender = gender;
    }

    pub fn set_date_of_birth(&mut self, date_of_birth: Option<chrono::NaiveDateTime>) {
        self.date_of_birth = date_of_birth;
    }

    pub fn set_hire_date(&mut self, hire_date: Option<chrono::NaiveDateTime>) {
        self.hire_date = hire_date;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_department_id(&mut self, department_id: Option<i32>) {
        self.department_id = department_id;
    }

    pub fn set_job_title(&mut self, job_title: Option<String>) {
        self.job_title = job_title;
    }

    pub fn set_salary(&mut self, salary: Option<i32>) {
        self.salary = salary;
    }

    pub fn set_manager_id(&mut self, manager_id: Option<i32>) {
        self.manager_id = manager_id;
    }

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn set_city(&mut self, city: Option<String>) {
        self.city = city;
    }

    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    pub fn set_postal_code(&mut self, postal_code: Option<String>) {
        self.postal_code = postal_code;
    }

    pub fn set_valid(&mut self, valid: Option<bool>) {
        self.valid = valid;
    }

    pub fn set_role_name(&mut self, role_name: Option<String>) {
        self.role_name = role_name;
    }

    pub fn set_role_id(&mut self, role_id: Option<i32>) {
        self.role_id = role_id;
    }
}
