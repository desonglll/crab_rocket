use rocket::serde::{Deserialize, Serialize};

use crate::modules::auth_param::AuthParam;

use super::pagination_request_param::PaginationParam;

/// ## Json Deserialize for passing request param.
/// ```
///pub struct RequestParam<PaginationParamGeneric, FilterParamGeneric> {
///    pub pagination: PaginationParamGeneric,
///    pub filter: Option<FilterParamGeneric>,
///}
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct RequestParam<FilterParamGeneric> {
    pub auth: Option<AuthParam>,
    pub pagination: Option<PaginationParam>,
    pub filter: Option<FilterParamGeneric>,
}

impl<T> RequestParam<T> {
    pub fn new(pagination: Option<PaginationParam>, filter: Option<T>) -> Self {
        if pagination.is_none() {
            Self {
                auth: Some(AuthParam::new()),
                pagination: Some(PaginationParam::default()),
                filter,
            }
        } else {
            Self {
                auth: Some(AuthParam::new()),
                pagination,
                filter,
            }
        }
    }

    pub fn demo() -> Self {
        Self {
            auth: Some(AuthParam::new()),
            pagination: Some(PaginationParam::default()),
            filter: None,
        }
    }
}
impl<T> Default for RequestParam<T> {
    fn default() -> Self {
        Self {
            auth: Some(AuthParam::new()),
            pagination: Some(PaginationParam::default()),
            filter: None,
        }
    }
}
