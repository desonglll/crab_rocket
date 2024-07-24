use obj_traits::service::service_crud::ServiceCRUD;

use crate::mappers::role_mapper::RoleMapper;
use crate::models::role::{PatchRole, PostRole, Role};
use crate::models::role_filter::RoleFilter;

pub struct RoleService {}

impl ServiceCRUD for RoleService {
    type Item = Role;
    type PostItem = PostRole;
    type PatchItem = PatchRole;
    type Filter = RoleFilter;
    type Mapper = RoleMapper;
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
            Ok(result) => println!("{result}"),
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
