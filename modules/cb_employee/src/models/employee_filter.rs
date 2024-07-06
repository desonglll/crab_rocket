use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeFilter {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth_min: Option<chrono::NaiveDateTime>,
    pub date_of_birth_max: Option<chrono::NaiveDateTime>,
    pub hire_date_min: Option<chrono::NaiveDateTime>,
    pub hire_date_max: Option<chrono::NaiveDateTime>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
    pub last_update_min: Option<chrono::NaiveDateTime>,
    pub last_update_max: Option<chrono::NaiveDateTime>,
    pub role_name: Option<String>,
    pub role_id: Option<i32>,
    // Pagination
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
impl EmployeeFilter {
    // 方法：从 JSON 字符串解析为 EmployeeFilter 实例
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[test]
fn test_from_json() {
    let json_data = r#"
        {
            "first_name": "John",
            "last_name": "Doe",
            "gender": "Male",
            "date_of_birth_min": "1990-01-01T00:00:00",
            "hire_date_max": "2020-01-01T00:00:00",
            "department_id": 10,
            "job_title": "Engineer",
            "salary_min": 50000,
            "valid": true
        }
    "#;

    match EmployeeFilter::from_json(json_data) {
        Ok(filter) => println!("{:?}", filter),
        Err(e) => eprintln!("Error parsing JSON: {:?}", e),
    }
}
