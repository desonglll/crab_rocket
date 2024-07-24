use crate::{request::request_param::RequestParam, response::data::Data};
use crab_rocket_schema::DbPool;
use rocket::State;

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait MapperCRUD {
    type Item: Default;
    type PostItem;
    type PatchItem;
    type Filter;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Self::Item>>, diesel::result::Error>;
    fn get_by_id(pool: &State<DbPool>, pid: i32)
        -> Result<Data<Self::Item>, diesel::result::Error>;
    fn add_single(
        pool: &State<DbPool>,
        obj: &Self::PostItem,
    ) -> Result<Data<Self::Item>, diesel::result::Error>;
    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Self::Item>, diesel::result::Error>;
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &Self::PatchItem,
    ) -> Result<Data<Self::Item>, diesel::result::Error>;
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Self::Item>>, diesel::result::Error>;
}
