use rocket::serde::{Deserialize, Serialize};

use super::pagination_request_param::PaginationParamTrait;

/// ## Json Deserialize for passing request param.
/// ```
///pub struct RequestParam<PaginationParamGeneric, FilterParamGeneric> {
///    pub pagination: PaginationParamGeneric,
///    pub filter: Option<FilterParamGeneric>,
///}
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct RequestParam<PaginationParamGeneric, FilterParamGeneric> {
    pub pagination: PaginationParamGeneric,
    pub filter: Option<FilterParamGeneric>,
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
