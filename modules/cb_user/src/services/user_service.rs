use crate::mappers::user_mapper::UserMapper;
use crate::models::user::{NewUser, PatchUser, User};
use crate::models::user_filter::UserFilter;
use crab_rocket_schema::establish_pg_connection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use std::error::Error;

pub struct UserService {}

impl ServiceCRUD<User, NewUser, PatchUser, RequestParam<PaginationParam, UserFilter>>
    for UserService
{
    fn get_all(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_all(&mut conn, param) {
                Ok(all_users) => Ok(all_users),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn get_by_id(pid: i32) -> Result<User, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::get_by_id(&mut conn, pid) {
                Ok(user) => Ok(user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn add_single(obj: &NewUser) -> Result<User, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::add_single(&mut conn, obj) {
                Ok(user) => Ok(user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_by_id(pid: i32) -> Result<User, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::delete_by_id(&mut conn, pid) {
                Ok(deleted_user) => Ok(deleted_user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn update_by_id(pid: i32, obj: &PatchUser) -> Result<User, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::update_by_id(&mut conn, pid, obj) {
                Ok(updated_user) => Ok(updated_user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn filter(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match UserMapper::filter(&mut conn, param) {
                Ok(all_users) => Ok(all_users),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::services::user_service::UserService;
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    #[test]
    fn test_insert_single_user() {
        use crate::models::user::NewUser;
        let user = NewUser::demo();
        match UserService::add_single(&user) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_users() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match UserService::get_all(&param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_user_by_id() {
        match UserService::get_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        match UserService::delete_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
