use crate::{
    models::follow::{Follow, NewFollow},
    routes::follow_param::FollowParam,
};

pub trait GetFollow {
    fn create_new_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>>;
    fn delete_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>>;
    fn get_all_follows() -> Result<Vec<Follow>, Box<dyn std::error::Error>>;
    fn filter_follows_by_params(
        params: &FollowParam,
    ) -> Result<Vec<Follow>, Box<dyn std::error::Error>>;
}
