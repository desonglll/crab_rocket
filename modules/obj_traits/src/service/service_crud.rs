use crate::response::data::Data;

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
