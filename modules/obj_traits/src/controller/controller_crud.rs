use crate::response::api_response::ApiResponse;
use crate::response::data::Data;

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait ControllerCRUD<T, U, V, P> {
    fn get_all(param: &P) -> Result<ApiResponse<Data<Vec<T>>>, Box<dyn std::error::Error>>;
    fn get_by_id(pid: i32) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn add_single(obj: &mut U) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn delete_by_id(pid: i32) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
    fn update_by_id(pid: i32, obj: &V) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
}
