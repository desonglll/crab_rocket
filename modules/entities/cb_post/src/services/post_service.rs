use crate::mappers::post_mapper::PostMapper;
use crate::models::post::{PatchPost, Post, PostPost};
use crate::models::post_filter::PostFilter;

use obj_traits::service::service_crud::ServiceCRUD;

pub struct PostService {}

impl ServiceCRUD for PostService {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Filter = PostFilter;
    type Mapper = PostMapper;
}

#[cfg(test)]
mod test {
    use crate::services::post_service::PostService;
    use crab_rocket_schema::{establish_pool, DbPool};
    use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
    use obj_traits::request::request_param::RequestParam;
    use obj_traits::service::service_crud::ServiceCRUD;
    use rocket::State;

    #[test]
    fn test_insert_single_post() {
        use crate::models::post::PostPost;
        let post = PostPost::demo();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PostService::add_single(pool, &post) {
            Ok(result) => println!("{result}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_all_posts() {
        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PostService::get_all(pool, &param) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_get_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PostService::get_by_id(pool, 1) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }

    #[test]
    fn test_delete_post_by_id() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match PostService::delete_by_id(pool, 2) {
            Ok(res) => println!("{res:?}"),
            Err(e) => println!("{e:?}"),
        }
    }
}
