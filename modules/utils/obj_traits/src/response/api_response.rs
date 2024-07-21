use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    code: i32,
    message: String,
    body: T,
}

impl<T: Default> ApiResponse<T> {
    pub fn new(code: i32, message: String, body: T) -> Self {
        Self {
            code,
            message,
            body,
        }
    }
    pub fn success(body: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            body,
        }
    }
    pub fn error(e: Box<dyn std::error::Error>) -> Self {
        Self {
            code: 204,
            message: e.to_string(),
            body: T::default(),
        }
    }
}
