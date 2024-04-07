use crate::{
    models::follow::{Follow, NewFollow},
    routes::models::follow_param::FollowParam,
    services::follow_service::GetFollow,
};

pub fn create_new_follow_controller(follow: &NewFollow) -> (i32, &'static str, Follow) {
    match Follow::create_new_follow(follow) {
        Ok(inserted_follow) => (200, "CREATE NEW FOLLOW OK", inserted_follow),
        Err(_) => (204, "CREATE NEW FOLLOW ERROR", Follow::new_empty()),
    }
}

pub fn get_all_follows_controller() -> (i32, &'static str, Vec<Follow>) {
    match Follow::get_all_follows() {
        Ok(all_follows) => (200, "GET ALL FOLLOWS OK", all_follows),
        Err(_) => (204, "GET ALL FOLLOWS ERROR", Vec::new()),
    }
}

pub fn delete_follow_controller(follow: &NewFollow) -> (i32, &'static str, Follow) {
    match Follow::delete_follow(follow) {
        Ok(deleted_follow) => (200, "DELETE FOLLOW OK", deleted_follow),
        Err(_) => (204, "DELETE FOLLOW ERROR", Follow::new_empty()),
    }
}

pub fn get_follows_by_params_controller(params: &FollowParam) -> (i32, &'static str, Vec<Follow>) {
    match Follow::get_follows_by_params(params) {
        Ok(follows) => (200, "GET FOLLOWS BT PARAMS OK", follows),
        Err(_) => (204, "GET FOLLOWS BY PARAMS ERROR", Vec::new()),
    }
}

#[cfg(test)]
mod test {
    use crate::models::follow::NewFollow;

    use super::create_new_follow_controller;

    #[test]
    fn test_create_new_follow_controller() {
        let follow = NewFollow::new(2, 3);
        let (code, message, inserted_follow) = create_new_follow_controller(&follow);
        println!("{code}\n{message}\n{inserted_follow:?}\n");
    }
}
