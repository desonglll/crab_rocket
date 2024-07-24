use std::error::Error;

use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::role_mapper::RoleMapper;
use crate::models::role::{PatchRole, PostRole, Role};
use crate::models::role_filter::RoleFilter;

pub struct RoleService {}

impl ServiceCRUD for RoleService {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Param = RequestParam<RoleFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<RoleFilter>,
    ) -> Result<Data<Vec<Role>>, Box<dyn Error>> {
        service_get_all::<Role, RoleMapper, RoleFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Role, Box<dyn Error>> {
        service_get_by_id::<Role, RoleMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostRole) -> Result<Role, Box<dyn Error>> {
        service_add_single::<Role, RoleMapper, PostRole>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Role, Box<dyn Error>> {
        service_delete_by_id::<Role, RoleMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchRole,
    ) -> Result<Role, Box<dyn Error>> {
        service_update_by_id::<Role, RoleMapper, PatchRole>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<RoleFilter>,
    ) -> Result<Data<Vec<Role>>, Box<dyn std::error::Error>> {
        service_filter::<Role, RoleMapper, RoleFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::role_service::RoleService;

    #[test]
    fn test_insert_single_role() {
        use crate::models::role::PostRole;
        let role = PostRole::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match RoleService::add_single(pool, &role) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_roles() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match RoleService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_role_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match RoleService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_role_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match RoleService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
