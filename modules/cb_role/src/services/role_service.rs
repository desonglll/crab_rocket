use crate::mappers::role_mapper::RoleMapper;
use crate::models::role::{NewRole, PatchRole, Role};
use crate::models::role_filter::RoleFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct RoleService {}

impl ServiceCRUD for RoleService {
    type Item = Role;
    type NewItem = NewRole;
    type PatchItem = PatchRole;
    type Param = RequestParam<PaginationParam, RoleFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, RoleFilter>,
    ) -> Result<Data<Vec<Role>>, Box<dyn Error>> {
        service_get_all::<Role, RoleMapper, RoleFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Role, Box<dyn Error>> {
        service_get_by_id::<Role, RoleMapper>(pid)
    }

    fn add_single(obj: &NewRole) -> Result<Role, Box<dyn Error>> {
        service_add_single::<Role, RoleMapper, NewRole>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Role, Box<dyn Error>> {
        service_delete_by_id::<Role, RoleMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchRole) -> Result<Role, Box<dyn Error>> {
        service_update_by_id::<Role, RoleMapper, PatchRole>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, RoleFilter>,
    ) -> Result<Data<Vec<Role>>, Box<dyn std::error::Error>> {
        service_filter::<Role, RoleMapper, RoleFilter>(param)
    }
}
