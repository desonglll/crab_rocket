use crab_rocket_schema::establish_pg_connection;
use obj_traits::{
    mapper::mapper_crud::MapperCRUD,
    request::{pagination_request_param::PaginationParam, request_param::RequestParam},
    service::service_crud::ServiceCRUD,
};

use crate::{
    mappers::post_mapper::PostMapper,
    models::{
        post::{NewPost, PatchPost, Post},
        post_filter::PostFilter,
    },
};

pub struct PostService {}
impl ServiceCRUD<Post, NewPost, PatchPost, RequestParam<PaginationParam, PostFilter>>
    for PostService
{
    fn get_all(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Post>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::get_all(&mut conn, param) {
                Ok(all_posts) => Ok(all_posts),
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

    fn get_by_id(pid: i32) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::get_by_id(&mut conn, pid) {
                Ok(post) => Ok(post),
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

    fn add_single(obj: &NewPost) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::add_single(&mut conn, obj) {
                Ok(inserted_post) => Ok(inserted_post),
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

    fn delete_by_id(pid: i32) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::delete_by_id(&mut conn, pid) {
                Ok(deleted_post) => Ok(deleted_post),
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

    fn update_by_id(pid: i32, obj: &PatchPost) -> Result<Post, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::update_by_id(&mut conn, pid, obj) {
                Ok(updated_post) => Ok(updated_post),
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
    fn filter(
        param: &RequestParam<PaginationParam, PostFilter>,
    ) -> Result<obj_traits::response::data::Data<Vec<Post>>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match PostMapper::filter(&mut conn, param) {
                Ok(all_posts) => Ok(all_posts),
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
        use crate::models::post::NewPost;
        let post = NewPost::demo();
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
