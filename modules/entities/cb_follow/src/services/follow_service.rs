use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;

use crate::mappers::follow_mapper::FollowMapper;
use crate::mappers::follow_mapper_trait::FollowMapperTrait;
use crate::models::follow::{Follow, PatchFollow, PostFollow};
use crate::models::follow_filter::FollowFilter;

use super::follow_service_trait::FollowServiceTrait;

pub struct FollowService {}

impl ServiceCRUD for FollowService {
    type Item = Follow;
    type PostItem = PostFollow;
    type PatchItem = PatchFollow;
    type Filter = FollowFilter;
    type Mapper = FollowMapper;
}

impl FollowServiceTrait<RequestParam<Follow, FollowFilter>> for FollowService {
    fn delete_follow_specifically(
        pool: &State<DbPool>,
        obj: &PostFollow,
    ) -> Result<Data<Follow>, Box<dyn std::error::Error>> {
        match FollowMapper::delete_follow_specifically(pool, obj) {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_followeds_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match FollowMapper::get_followeds_by_user_id(pool, uid, param) {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_followings_by_user_id(
        pool: &State<DbPool>,
        uid: i32,
        param: &RequestParam<Follow, FollowFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Follow>>, Box<dyn std::error::Error>> {
        match FollowMapper::get_followings_by_user_id(pool, uid, param) {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;

    use crate::services::follow_service::FollowService;

    #[test]
    fn test_insert_single_follow() {
        use crate::models::follow::PostFollow;
        let follow = PostFollow::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::add_single(pool, &follow) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_follows() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_follow_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_follow_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match FollowService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
