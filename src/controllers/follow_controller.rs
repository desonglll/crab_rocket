use crate::{
    models::follow::{Follow, NewFollow},
    routes::models::follow_param::FollowParam,
    services::follow_service::GetFollow,
};

pub fn create_new_follow_controller(follow: &NewFollow) -> (i32, String, Follow) {
    match Follow::create_new_follow(follow) {
        Ok(inserted_follow) => (200, String::from("CREATE NEW FOLLOW OK"), inserted_follow),
        Err(e) => (204, e.to_string(), Follow::default()),
    }
}

pub fn get_all_follows_controller() -> (i32, String, Vec<Follow>) {
    match Follow::get_all_follows() {
        Ok(all_follows) => (200, String::from("GET ALL FOLLOWS OK"), all_follows),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

pub fn delete_follow_controller(follow: &NewFollow) -> (i32, String, Follow) {
    match Follow::delete_follow(follow) {
        Ok(deleted_follow) => (200, String::from("DELETE FOLLOW OK"), deleted_follow),
        Err(e) => (204, e.to_string(), Follow::default()),
    }
}

pub fn get_follows_by_params_controller(params: &FollowParam) -> (i32, String, Vec<Follow>) {
    match Follow::filter_follows_by_params(params) {
        Ok(follows) => (200, String::from("GET FOLLOWS BT PARAMS OK"), follows),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

#[cfg(test)]
mod test {
    use crate::models::follow::NewFollow;

    use super::create_new_follow_controller;

    #[test]
    fn test_create_new_follow_controller() {
        let follow = NewFollow::new(2, 3, None);
        let (code, message, inserted_follow) = create_new_follow_controller(&follow);
        println!("{code}\n{message}\n{inserted_follow:?}\n");
    }
}
