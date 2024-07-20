use crate::request::pagination_request_param::Pagination;
use rocket::serde::Serialize;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Data<T> {
    data: T,
    pagination: Pagination,
}

impl<T> Data<T> {
    pub fn set_data(&mut self, data: T) {
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
