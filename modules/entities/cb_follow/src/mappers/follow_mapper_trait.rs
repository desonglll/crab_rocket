use diesel::PgConnection;

use obj_traits::{mapper::mapper_crud::MapperCRUD, response::data::Data};

use crate::models::follow::{Follow, PostFollow};

pub trait FollowMapperTrait: MapperCRUD {
    fn delete_follow_specifically(
        conn: &mut PgConnection,
        obj: &PostFollow,
    ) -> Result<Follow, diesel::result::Error>;
    fn get_followings_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &Self::Param,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
    fn get_followeds_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &Self::Param,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
}
