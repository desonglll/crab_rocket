use rocket::serde::Serialize;

use crate::request::pagination_request_param::Pagination;

#[derive(Serialize, Default, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Data<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

impl<T> Data<T> {
    pub fn new(data: T, pagination: Option<Pagination>) -> Self {
        Self {
            data,
            pagination,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Data<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.data, self.pagination.clone().unwrap_or_default())
    }
}
