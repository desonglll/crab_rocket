use crate::models::role::{PatchRole, PostRole, Role};
use crate::models::role_filter::RoleFilter;
use crate::services::role_service::RoleService;
use obj_traits::controller::controller_crud::{
    controller_add_single, controller_delete_by_id, controller_filter, controller_get_all,
    controller_get_by_id, controller_update_by_id, ControllerCRUD,
};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::api_response::ApiResponse;
use obj_traits::response::data::Data;
use std::error::Error;

pub struct RoleController {}

impl ControllerCRUD for RoleController {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Param = RequestParam<RoleFilter>;
    fn get_all(
        param: &RequestParam<RoleFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn Error>> {
        controller_get_all::<Self::Item, RoleService, RoleFilter>(param)
    }

    fn get_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_get_by_id::<Self::Item, RoleService>(pid)
    }

    fn add_single(obj: &mut PostRole) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_add_single::<Self::Item, RoleService, PostRole>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_delete_by_id::<Self::Item, RoleService>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchRole) -> Result<ApiResponse<Self::Item>, Box<dyn Error>> {
        controller_update_by_id::<Self::Item, RoleService, PatchRole>(pid, obj)
    }
    fn filter(
        param: &RequestParam<RoleFilter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        controller_filter::<Self::Item, RoleService, RoleFilter>(param)
    }
}
