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
pub struct RequestParam<Obj: Default, FilterParamGeneric> {
    pub auth: Option<AuthParam>,
    pub pagination: Option<PaginationParam>,
    pub filter: Option<FilterParamGeneric>,
    pub data: Option<Obj>,
}

impl<T: Default, U> RequestParam<T, U> {
    pub fn new(pagination: Option<PaginationParam>, filter: Option<U>) -> Self {
        if pagination.is_none() {
            Self {
                auth: Some(AuthParam::new()),
                pagination: Some(PaginationParam::default()),
                filter,
                data: Some(T::default()),
            }
        } else {
            Self {
                auth: Some(AuthParam::new()),
                pagination,
                filter,
                data: Some(T::default()),
            }
        }
    }

    pub fn demo() -> Self {
        Self {
            auth: Some(AuthParam::new()),
            pagination: Some(PaginationParam::default()),
            filter: None,
            data: Some(T::default()),
        }
    }
}
impl<T: Default, U> Default for RequestParam<T, U> {
    fn default() -> Self {
        Self {
            auth: Some(AuthParam::new()),
            pagination: Some(PaginationParam::default()),
            filter: None,
            data: Some(T::default()),
        }
    }
}
