use std::error::Error;

use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};

use crate::mappers::user_mapper::UserMapper;
use crate::models::user::{PatchUser, PostUser, User};
use crate::models::user_filter::UserFilter;

pub struct UserService {}
impl UserService {
    pub fn get_by_username(pool: &State<DbPool>, username: String) -> Result<User, Box<dyn Error>> {
        let mut conn = establish_pg_connection(pool).expect("Error establish_pg_connection");
        match UserMapper::get_by_username(&mut conn, username) {
            Ok(user) => Ok(user),
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl ServiceCRUD for UserService {
    type Item = User;
    type PostItem = PostUser;
    type PatchItem = PatchUser;
    type Param = RequestParam<UserFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn Error>> {
        service_get_all::<User, UserMapper, UserFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<User, Box<dyn Error>> {
        service_get_by_id::<User, UserMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostUser) -> Result<User, Box<dyn Error>> {
        service_add_single::<User, UserMapper, PostUser>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<User, Box<dyn Error>> {
        service_delete_by_id::<User, UserMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchUser,
    ) -> Result<User, Box<dyn Error>> {
        service_update_by_id::<User, UserMapper, PatchUser>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn std::error::Error>> {
        service_filter::<User, UserMapper, UserFilter>(pool, param)
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::user_service::UserService;

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
