use crate::mappers::permission_mapper::PermissionMapper;
use crate::models::permission::{PatchPermission, Permission, PostPermission};
use crate::models::permission_filter::PermissionFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct PermissionService {}

impl ServiceCRUD for PermissionService {
    type Item = Permission;
    type PostItem = PostPermission;
    type PatchItem = PatchPermission;
    type Filter = PermissionFilter;
    type Mapper = PermissionMapper;
}

#[cfg(test)]
mod test {
    use crate::services::permission_service::PermissionService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_permission() {
        use crate::models::permission::PostPermission;
        let permission = PostPermission::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PermissionService::add_single(pool, &permission) {
            Ok(result) => println!("{result}"),
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
