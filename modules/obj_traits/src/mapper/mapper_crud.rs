use crate::response::data::Data;
use diesel::PgConnection;

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
    fn filter(conn: &mut PgConnection, param: &P) -> Result<Data<Vec<T>>, diesel::result::Error>;
}
