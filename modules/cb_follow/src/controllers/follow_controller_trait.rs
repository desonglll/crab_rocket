use obj_traits::response::{api_response::ApiResponse, data::Data};

use crate::models::follow::{Follow, PostFollow};

pub trait FollowControllerTrait<P> {
    fn delete_follow_specifically(
        obj: &PostFollow,
    ) -> Result<ApiResponse<Follow>, Box<dyn std::error::Error>>;

    fn get_followings_by_user_id(
        uid: i32,
        param: &P,
    ) -> Result<ApiResponse<Data<Vec<Follow>>>, Box<dyn std::error::Error>>;
    fn get_followeds_by_user_id(
        uid: i32,
        param: &P,
    ) -> Result<ApiResponse<Data<Vec<Follow>>>, Box<dyn std::error::Error>>;
}
