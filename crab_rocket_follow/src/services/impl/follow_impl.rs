use crate::{
    mappers::follow_mapper,
    models::follow::{Follow, NewFollow},
    routes::follow_param::FollowParam,
    services::follow_service::GetFollow,
};
use crab_rocket_schema::establish_pg_connection;

impl GetFollow for Follow {
    fn create_new_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match follow_mapper::create_new_follow(&mut conn, &follow) {
                    Ok(inserted_follow) => Ok(inserted_follow),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_all_follows() -> Result<Vec<Follow>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match follow_mapper::fetch_all_follows(&mut conn) {
                    Ok(all_follows) => Ok(all_follows),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn delete_follow(follow: &NewFollow) -> Result<Follow, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match follow_mapper::delete_follow(&mut conn, follow) {
                    Ok(deleted_follow) => Ok(deleted_follow),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn filter_follows_by_params(
        params: &FollowParam,
    ) -> Result<Vec<Follow>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match follow_mapper::fetch_follows_by_params(&mut conn, params) {
                    Ok(follows) => Ok(follows),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}
