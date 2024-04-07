use crate::models::follow::{Follow, NewFollow};

pub trait GetFollow {
    fn create_new_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>>;
    fn delete_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>>;
    fn get_all_follows() -> Result<Vec<Follow>, Box<dyn std::error::Error>>;
    fn get_following_by_id(following_id: i32) -> Result<Vec<Follow>, Box<dyn std::error::Error>>;
    fn get_followed_by_id(followed_id: i32) -> Result<Vec<Follow>, Box<dyn std::error::Error>>;
}
