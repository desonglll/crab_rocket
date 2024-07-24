use crab_rocket_schema::DbPool;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam, response::data::Data,
};
use rocket::State;

use crate::models::{
    follow::{Follow, PostFollow},
    follow_filter::FollowFilter,
};

pub trait FollowMapperTrait: MapperCRUD {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Data<Follow>, diesel::result::Error>;
    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
}
