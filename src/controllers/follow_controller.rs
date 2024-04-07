use crate::{
    models::follow::{Follow, NewFollow},
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

pub fn get_follows_by_user_id_controller(
    following_id: i32,
    followed_id: i32,
) -> (i32, &'static str, Vec<Follow>) {
    if following_id != 0 && followed_id == 0 {
        match Follow::get_following_by_id(following_id) {
            Ok(followings) => (200, "GET FOLLOWING BY ID OK", followings),
            Err(_) => (204, "GET FOLLOWING BY ID ERROR", Vec::new()),
        }
    } else if following_id == 0 && followed_id != 0 {
        match Follow::get_followed_by_id(followed_id) {
            Ok(followeds) => (200, "GET FOLLOWEDS BY ID OK", followeds),
            Err(_) => (204, "GET FOLLOWEDS BY ID ERROR", Vec::new()),
        }
    } else {
        (204, "GET FOLLOWS ERROR", Vec::new())
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
