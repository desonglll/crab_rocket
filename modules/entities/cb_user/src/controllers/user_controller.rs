use crate::models::user::{PatchUser, PostUser, User};
use crate::models::user_filter::UserFilter;
use crate::services::user_service::UserService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct UserController {}

impl ControllerCRUD for UserController {
    type Item = User;
    type PostItem = PostUser;
    type PatchItem = PatchUser;
    type Service = UserService;
    type Filter = UserFilter;
}
