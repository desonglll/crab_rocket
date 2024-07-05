use diesel::PgConnection;
use obj_traits::response::data::Data;

use crate::models::follow::{Follow, NewFollow};

pub trait FollowMapperTrait<P> {
    fn delete_follow_specifically(
        conn: &mut PgConnection,
        obj: &NewFollow,
    ) -> Result<Follow, diesel::result::Error>;
    fn get_followings_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
    fn get_followeds_by_user_id(
        conn: &mut PgConnection,
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, diesel::result::Error>;
}
