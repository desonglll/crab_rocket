use crate::mappers::permission_mapper::PermissionMapper;
use crate::models::permission::{PostPermission, PatchPermission, Permission};
use crate::models::permission_filter::PermissionFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct PermissionService {}

impl ServiceCRUD for PermissionService {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Param = RequestParam<PaginationParam, PermissionFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, Box<dyn Error>> {
        service_get_all::<Permission, PermissionMapper, PermissionFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Permission, Box<dyn Error>> {
        service_get_by_id::<Permission, PermissionMapper>(pid)
    }

    fn add_single(obj: &PostPermission) -> Result<Permission, Box<dyn Error>> {
        service_add_single::<Permission, PermissionMapper, PostPermission>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Permission, Box<dyn Error>> {
        service_delete_by_id::<Permission, PermissionMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchPermission) -> Result<Permission, Box<dyn Error>> {
        service_update_by_id::<Permission, PermissionMapper, PatchPermission>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, Box<dyn std::error::Error>> {
        service_filter::<Permission, PermissionMapper, PermissionFilter>(param)
    }
}
