use obj_traits::response::data::Data;

use crate::models::follow::{Follow, NewFollow};

pub trait FollowServiceTrait<P> {
    fn delete_follow_specifically(obj: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>>;
    fn get_followings_by_user_id(
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>>;
    fn get_followeds_by_user_id(
        uid: i32,
        param: &P,
    ) -> Result<Data<Vec<Follow>>, Box<dyn std::error::Error>>;
}
