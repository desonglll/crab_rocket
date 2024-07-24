use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::permission_mapper::PermissionMapper;
use crate::models::permission::{PatchPermission, Permission, PostPermission};
use crate::models::permission_filter::PermissionFilter;

pub struct PermissionService {}

impl ServiceCRUD for PermissionService {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Param = RequestParam<PermissionFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, Box<dyn Error>> {
        service_get_all::<Permission, PermissionMapper, PermissionFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Permission, Box<dyn Error>> {
        service_get_by_id::<Permission, PermissionMapper>(pool, pid)
    }

    fn add_single(
        pool: &State<DbPool>,
        obj: &PostPermission,
    ) -> Result<Permission, Box<dyn Error>> {
        service_add_single::<Permission, PermissionMapper, PostPermission>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Permission, Box<dyn Error>> {
        service_delete_by_id::<Permission, PermissionMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchPermission,
    ) -> Result<Permission, Box<dyn Error>> {
        service_update_by_id::<Permission, PermissionMapper, PatchPermission>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<PermissionFilter>,
    ) -> Result<Data<Vec<Permission>>, Box<dyn std::error::Error>> {
        service_filter::<Permission, PermissionMapper, PermissionFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::permission_service::PermissionService;

    #[test]
    fn test_insert_single_permission() {
        use crate::models::permission::PostPermission;
        let permission = PostPermission::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PermissionService::add_single(pool, &permission) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_permissions() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PermissionService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_permission_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PermissionService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_permission_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PermissionService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
