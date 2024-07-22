use crate::mappers::follow_mapper::FollowMapper;
use crate::mappers::follow_mapper_trait::FollowMapperTrait;
use crate::models::follow::{Follow, PatchFollow, PostFollow};
use crate::models::follow_filter::FollowFilter;
use crab_rocket_schema::{establish_pg_connection, DbPool};
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::{
    service_add_single, service_delete_by_id, service_filter, service_get_all, service_get_by_id,
    service_update_by_id, ServiceCRUD,
};
use rocket::State;
use std::error::Error;

use super::follow_service_trait::FollowServiceTrait;

pub struct FollowService {}

impl ServiceCRUD for FollowService {
    type Item = Follow;
    type PostItem = PostFollow;
    type PatchItem = PatchFollow;
    type Param = RequestParam<FollowFilter>;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, Box<dyn Error>> {
        service_get_all::<Follow, FollowMapper, FollowFilter>(pool, param)
    }
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Follow, Box<dyn Error>> {
        service_get_by_id::<Follow, FollowMapper>(pool, pid)
    }

    fn add_single(pool: &State<DbPool>, obj: &PostFollow) -> Result<Follow, Box<dyn Error>> {
        service_add_single::<Follow, FollowMapper, PostFollow>(pool, obj)
    }

    fn delete_by_id(pool: &State<DbPool>, pid: i32) -> Result<Follow, Box<dyn Error>> {
        service_delete_by_id::<Follow, FollowMapper>(pool, pid)
    }

    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &PatchFollow,
    ) -> Result<Follow, Box<dyn Error>> {
        service_update_by_id::<Follow, FollowMapper, PatchFollow>(pool, pid, obj)
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        service_filter::<Follow, FollowMapper, FollowFilter>(pool, param)
    }
}

impl FollowServiceTrait<RequestParam<FollowFilter>> for FollowService {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match FollowMapper::delete_follow_specifically(&mut conn, obj) {
                Ok(data) => Ok(data),
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

    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<FollowFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match FollowMapper::get_followeds_by_user_id(&mut conn, uid, param) {
                Ok(data) => Ok(data),
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

    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<FollowFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match FollowMapper::get_followings_by_user_id(&mut conn, uid, param) {
                Ok(data) => Ok(data),
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
    use crate::services::follow_service::FollowService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_follow() {
        use crate::models::follow::PostFollow;
        let follow = PostFollow::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::add_single(pool, &follow) {
            Ok(result) => println!("{result:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_follows() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_follow_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_follow_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
