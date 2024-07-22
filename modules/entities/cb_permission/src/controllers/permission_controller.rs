use crate::models::permission::{PatchPermission, Permission, PostPermission};
use crate::models::permission_filter::PermissionFilter;
use crate::services::permission_service::PermissionService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct PermissionController {}

impl ControllerCRUD for PermissionController {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Param = RequestParam<PermissionFilter>;
    fn get_all(param: &Self::Param) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, PermissionService, PermissionFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, PermissionService>(pid)
    }

    fn add_single(obj: &mut PostPermission) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, PermissionService, PostPermission>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, PermissionService>(pid)
    }

    fn update_by_id(
        pid: i32,
        obj: &PatchPermission,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, PermissionService, PatchPermission>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PermissionFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, PermissionService, PermissionFilter>(param)
    }
}
