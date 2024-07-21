use std::error::Error;

use obj_traits::{
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    response::data::Data,
    service::service_crud::{
        service_add_single, service_delete_by_id, service_filter, service_get_all,
        service_get_by_id, service_update_by_id, ServiceCRUD,
    },
};

use crate::{
    mappers::post_mapper::PostMapper,
    models::{
        post::{PostPost, PatchPost, Post},
        post_filter::PostFilter,
    },
};

pub struct PostService {}
impl ServiceCRUD for PostService {
    type Item = Post;
    type PostItem = PostPost;
    type PatchItem = PatchPost;
    type Param = RequestParam<PaginationParam, PostFilter>;
    fn get_all(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<Data<Vec<Post>>, Box<dyn Error>> {
        service_get_all::<Post, PostMapper, PostFilter>(param)
    }
    fn get_by_id(pid: i32) -> Result<Post, Box<dyn Error>> {
        service_get_by_id::<Post, PostMapper>(pid)
    }

    fn add_single(obj: &PostPost) -> Result<Post, Box<dyn Error>> {
        service_add_single::<Post, PostMapper, PostPost>(obj)
    }

    fn delete_by_id(pid: i32) -> Result<Post, Box<dyn Error>> {
        service_delete_by_id::<Post, PostMapper>(pid)
    }

    fn update_by_id(pid: i32, obj: &PatchPost) -> Result<Post, Box<dyn Error>> {
        service_update_by_id::<Post, PostMapper, PatchPost>(pid, obj)
    }
    fn filter(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<Data<Vec<Post>>, Box<dyn std::error::Error>> {
        service_filter::<Post, PostMapper, PostFilter>(param)
    }
}

#[cfg(test)]
mod test {
    use obj_traits::{
        request::{
            pagination_request_param::{PaginationParam, PaginationParamTrait},
            request_param::RequestParam,
        },
        service::service_crud::ServiceCRUD,
    };

    use crate::services::post_service::PostService;

    #[test]
    fn test_insert_single_post() {
        use crate::models::post::PostPost;
        let post = PostPost::demo();
        let inserted_post = PostService::add_single(&post);
        println!("{inserted_post:?}");
    }
    #[test]
    fn test_get_all_posts() {
        let param = RequestParam::new(PaginationParam::demo(), None);
        match PostService::get_all(&param) {
            Ok(all_posts) => {
                println!("{all_posts}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_get_post_by_id() {
        match PostService::get_by_id(1) {
            Ok(post) => {
                println!("{post:?}");
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_delete_post_by_id() {
        match PostService::delete_by_id(4) {
            Ok(deleted_post) => {
                println!("{deleted_post:?}");
            }
            Err(_) => (),
        }
    }
}
