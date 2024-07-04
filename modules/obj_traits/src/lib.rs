use diesel::PgConnection;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct PaginationRequestParam {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl PaginationRequestParam {
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RequestParam<P> {
    pub pagination: P,
}

impl<P> RequestParam<P> {
    pub fn new(pagination: P) -> Self {
        Self { pagination }
    }
}

#[derive(Serialize, Default, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Pagination {
    page: i32,
    per_page: i32,
    total_pages: i32,
    count: i32,
    next: Option<String>,
    previous: Option<String>,
}

impl Pagination {
    pub fn new(
        page: i32,
        per_page: i32,
        total_pages: i32,
        count: i32,
        next: Option<String>,
        previous: Option<String>,
    ) -> Self {
        Self {
            page,
            per_page,
            total_pages,
            count,
            next,
            previous,
        }
    }
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the pagination information into a string
        write!(f, "Page {} of {} | Showing {} items", self.page, self.total_pages, self.count)?;

        // Optionally add previous and next links
        if let Some(prev) = &self.previous {
            write!(f, " | Previous: {}", prev)?;
        }
        if let Some(next) = &self.next {
            write!(f, " | Next: {}", next)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Data<T> {
    data: T,
    pagination: Pagination,
}

impl<T> Data<T> {
    pub fn set_body(&mut self, data: T) {
        self.data = data;
    }
    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = pagination;
    }
}

impl<T> Data<T> {
    pub fn data(&self) -> &T {
        &self.data
    }
    pub fn pagination(&self) -> &Pagination {
        &self.pagination
    }
}


impl<T> Data<T> {
    pub fn new(data: T, pagination: Pagination) -> Self {
        Self {
            data,
            pagination,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Data<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.data, self.pagination)
    }
}

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

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait MapperCRUD<T, U, V, P> {
    fn get_all(conn: &mut PgConnection, param: &P) -> Result<Data<Vec<T>>, diesel::result::Error>;
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<T, diesel::result::Error>;
    fn add_single(conn: &mut PgConnection, obj: &U) -> Result<T, diesel::result::Error>;
    fn delete_by_id(conn: &mut PgConnection, pid: i32) -> Result<T, diesel::result::Error>;
    fn update_by_id(conn: &mut PgConnection, pid: i32, obj: &V)
                    -> Result<T, diesel::result::Error>;
}

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait ServiceCRUD<T, U, V, P> {
    fn get_all(param: &P) -> Result<Data<Vec<T>>, Box<dyn std::error::Error>>;
    fn get_by_id(pid: i32) -> Result<T, Box<dyn std::error::Error>>;
    fn add_single(obj: &U) -> Result<T, Box<dyn std::error::Error>>;
    fn delete_by_id(pid: i32) -> Result<T, Box<dyn std::error::Error>>;
    fn update_by_id(pid: i32, obj: &V) -> Result<T, Box<dyn std::error::Error>>;
}

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait ControllerCRUD<T, U, V, P> {
    fn get_all(param: &P) -> Result<ApiResponse<Data<Vec<T>>>, Box<dyn std::error::Error>>;
    fn get_by_id(pid: i32) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn add_single(obj: &mut U) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn delete_by_id(pid: i32) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn update_by_id(pid: i32, obj: &V) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
}
