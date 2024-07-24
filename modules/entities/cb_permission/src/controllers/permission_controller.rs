use crate::models::permission::{PatchPermission, Permission, PostPermission};
use crate::models::permission_filter::PermissionFilter;
use crate::services::permission_service::PermissionService;
use obj_traits::controller::controller_crud::ControllerCRUD;

pub struct PermissionController {}

impl ControllerCRUD for PermissionController {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Filter = PermissionFilter;
    type Service = PermissionService;
}
