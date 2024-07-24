use obj_traits::controller::controller_crud::ControllerCRUD;

use crate::models::role::{PatchRole, PostRole, Role};
use crate::models::role_filter::RoleFilter;
use crate::services::role_service::RoleService;

pub struct RoleController {}

impl ControllerCRUD for RoleController {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Filter = RoleFilter;
    type Service = RoleService;
}
