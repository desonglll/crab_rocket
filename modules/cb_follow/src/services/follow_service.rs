use std::error::Error;

use crab_rocket_schema::establish_pg_connection;
use obj_traits::{
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    response::data::Data,
    service::service_crud::{
        service_add_single, service_delete_by_id, service_filter, service_get_all,
        service_get_by_id, service_update_by_id, ServiceCRUD,
    },
};

use crate::{
    mappers::{follow_mapper::FollowMapper, follow_mapper_trait::FollowMapperTrait},
    models::{
        follow::{Follow, NewFollow, PatchFollow},
        follow_filter::FollowFilter,
    },
};

use super::follow_service_trait::FollowServiceTrait;

pub struct FollowService {}

impl ServiceCRUD for FollowService {
    type Item = Follow;
    type NewItem = NewFollow;
    type PatchItem = PatchFollow;
    type Param = RequestParam<PaginationParam, FollowFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, Box<dyn Error>> {
        service_get_all::<Follow, FollowMapper, FollowFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Follow, Box<dyn Error>> {
        service_get_by_id::<Follow, FollowMapper>(pid)
    }

    fn add_single(obj: &NewFollow) -> Result<Follow, Box<dyn Error>> {
        service_add_single::<Follow, FollowMapper, NewFollow>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Follow, Box<dyn Error>> {
        service_delete_by_id::<Follow, FollowMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchFollow) -> Result<Follow, Box<dyn Error>> {
        service_update_by_id::<Follow, FollowMapper, PatchFollow>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        service_filter::<Follow, FollowMapper, FollowFilter>(param)
    }
}

impl FollowServiceTrait<RequestParam<PaginationParam, FollowFilter>> for FollowService {
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
        param: &RequestParam<PaginationParam, FollowFilter>,
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
        param: &RequestParam<PaginationParam, FollowFilter>,
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
