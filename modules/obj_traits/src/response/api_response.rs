use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    code: String,
    message: String,
    body: T,
}

impl<T: Default> ApiResponse<T> {
    pub fn new(code: String, message: String, body: T) -> Self {
        Self {
            code,
            message,
            body,
        }
    }
    pub fn success(body: T) -> Self {
        Self {
            code: "200".to_string(),
            message: "Success".to_string(),
            body,
        }
    }
    pub fn error() -> Self {
        Self {
            code: "200".to_string(),
            message: "Error".to_string(),
            body: T::default(),
        }
    }
}
