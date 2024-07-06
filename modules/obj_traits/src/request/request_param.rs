use rocket::serde::{Deserialize, Serialize};

use super::pagination_request_param::PaginationParamTrait;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RequestParam<P, T> {
    pub pagination: P,
    pub filter: Option<T>,
}

impl<P: PaginationParamTrait, T> RequestParam<P, T> {
    pub fn new(pagination: P, filter: Option<T>) -> Self {
        Self {
            pagination,
            filter,
        }
    }
    pub fn default() -> Self {
        Self {
            pagination: P::default(),
            filter: None,
        }
    }
}
