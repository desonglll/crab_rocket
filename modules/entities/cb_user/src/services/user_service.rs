use crate::mappers::user_mapper::UserMapper;
use crate::models::user::{PatchUser, PostUser, User};
use crate::models::user_filter::UserFilter;
use crab_rocket_schema::DbPool;
use obj_traits::service::service_crud::ServiceCRUD;
use rocket::State;
use std::error::Error;

pub struct UserService {}
impl UserService {
    pub fn get_by_username(pool: &State<DbPool>, username: String) -> Result<User, Box<dyn Error>> {
        match UserMapper::get_by_username(pool, username) {
            Ok(user) => Ok(user),
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl ServiceCRUD for UserService {
    type Item = User;
    type PostItem = PostUser;
    type PatchItem = PatchUser;
    type Filter = UserFilter;
    type Mapper = UserMapper;
}

#[cfg(test)]
mod test {
    use crate::services::user_service::UserService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_user() {
        use crate::models::user::PostUser;
        let user = PostUser::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserService::add_single(pool, &user) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_users() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_user_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match UserService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
