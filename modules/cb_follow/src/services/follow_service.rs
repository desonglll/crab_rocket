use crab_rocket_schema::establish_pg_connection;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    service::service_crud::ServiceCRUD,
};

use crate::{
    mappers::{follow_mapper::FollowMapper, follow_mapper_trait::FollowMapperTrait},
    models::follow::{Follow, NewFollow, PatchFollow},
};

use super::follow_service_trait::FollowServiceTrait;

pub struct FollowService {}

impl ServiceCRUD<Follow, NewFollow, PatchFollow, RequestParam<PaginationParam>> for FollowService {
    fn get_all(
        param: &RequestParam<PaginationParam>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::get_all(&mut conn, param) {
                Ok(all_follows) => Ok(all_follows),
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

    fn get_by_id(pid: i32) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::get_by_id(&mut conn, pid) {
                Ok(follow) => Ok(follow),
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

    fn add_single(obj: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::add_single(&mut conn, obj) {
                Ok(inserted_follow) => Ok(inserted_follow),
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

    fn delete_by_id(pid: i32) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::delete_by_id(&mut conn, pid) {
                Ok(deleted_follow) => Ok(deleted_follow),
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

    fn update_by_id(pid: i32, obj: &PatchFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match FollowMapper::update_by_id(&mut conn, pid, obj) {
                Ok(updated_follow) => Ok(updated_follow),
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

impl FollowServiceTrait<RequestParam<PaginationParam>> for FollowService {
    fn delete_follow_specifically(obj: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
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
        uid: i32,
        param: &RequestParam<PaginationParam>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
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
        uid: i32,
        param: &RequestParam<PaginationParam>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
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
