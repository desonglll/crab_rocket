use diesel::PgConnection;

use crate::response::data::Data;

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait MapperCRUD {
    type Item;
    type PostItem;
    type PatchItem;
    type Param;
    fn get_all(
        conn: &mut PgConnection,
        param: &Self::Param,
    ) -> Result<Data<Vec<Self::Item>>, diesel::result::Error>;
    fn get_by_id(conn: &mut PgConnection, pid: i32) -> Result<Self::Item, diesel::result::Error>;
    fn add_single(
        conn: &mut PgConnection,
        obj: &Self::PostItem,
    ) -> Result<Self::Item, diesel::result::Error>;
    fn delete_by_id(conn: &mut PgConnection, pid: i32)
        -> Result<Self::Item, diesel::result::Error>;
    fn update_by_id(
        conn: &mut PgConnection,
        pid: i32,
        obj: &Self::PatchItem,
    ) -> Result<Self::Item, diesel::result::Error>;
    fn filter(
        conn: &mut PgConnection,
        param: &Self::Param,
    ) -> Result<Data<Vec<Self::Item>>, diesel::result::Error>;
}
