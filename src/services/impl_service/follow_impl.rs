use crate::{
    establish_pg_connection,
    mappers::follow_mapper,
    models::follow::{Follow, NewFollow},
    services::follow_service::GetFollow,
};

impl GetFollow for Follow {
    fn create_new_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match follow_mapper::create_new_follow(&mut conn, &follow) {
                Ok(inserted_follow) => Ok(inserted_follow),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_all_follows() -> Result<Vec<Follow>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match follow_mapper::fetch_all_follows(&mut conn) {
                Ok(all_follows) => Ok(all_follows),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match follow_mapper::delete_follow(&mut conn, follow) {
                Ok(deleted_follow) => Ok(deleted_follow),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_following_by_id(following_id: i32) -> Result<Vec<Follow>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match follow_mapper::fetch_following_by_id(&mut conn, following_id) {
                Ok(followings) => Ok(followings),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn get_followed_by_id(followed_id: i32) -> Result<Vec<Follow>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match follow_mapper::fetch_followed_by_id(&mut conn, followed_id) {
                Ok(followeds) => Ok(followeds),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}
