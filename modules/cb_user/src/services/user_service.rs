use crate::mappers::user_mapper::UserMapper;
use crate::models::user::{NewUser, PatchUser, User};
use crate::models::user_filter::UserFilter;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use std::error::Error;

pub struct UserService {}

impl ServiceCRUD for UserService {
    type Item = User;
    type NewItem = NewUser;
    type PatchItem = PatchUser;
    type Param = RequestParam<PaginationParam, UserFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn Error>> {
        service_get_all::<User, UserMapper, UserFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<User, Box<dyn Error>> {
        service_get_by_id::<User, UserMapper>(pid)
    }

    fn add_single(obj: &NewUser) -> Result<User, Box<dyn Error>> {
        service_add_single::<User, UserMapper, NewUser>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<User, Box<dyn Error>> {
        service_delete_by_id::<User, UserMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchUser) -> Result<User, Box<dyn Error>> {
        service_update_by_id::<User, UserMapper, PatchUser>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, UserFilter>,
    ) -> Result<Data<Vec<User>>, Box<dyn std::error::Error>> {
        service_filter::<User, UserMapper, UserFilter>(param)
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
