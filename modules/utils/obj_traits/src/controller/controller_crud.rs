use rocket::State;

use crab_rocket_schema::DbPool;
use session::models::session::Session;

use crate::request::request_param::RequestParam;
use crate::response::api_response::ApiResponse;
use crate::response::data::Data;
use crate::service::service_crud::ServiceCRUD;

pub trait ControllerCRUD {
    type Item: Default;
    type PostItem;
    type PatchItem;
    type Filter;
    type Service: ServiceCRUD<
        Item = Self::Item,
        PostItem = Self::PostItem,
        PatchItem = Self::PatchItem,
        Filter = Self::Filter,
    >;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::get_all(pool, param) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Self::Item>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::get_by_id(pool, pid) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
    fn add_single(
        pool: &State<DbPool>,
        obj: &mut Self::PostItem,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Self::Item>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::add_single(pool, obj) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Self::Item>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::delete_by_id(pool, pid) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &Self::PatchItem,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Self::Item>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::update_by_id(pool, pid, obj) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>> {
        let session_id = param.auth.unwrap().session_id;
        let session = Session::get_session_by_id(pool, session_id);
        match session {
            Ok(existing_session) => match existing_session.is_valid(pool) {
                Ok(_) => match Self::Service::filter(pool, param) {
                    Ok(data) => Ok(ApiResponse::success(data)),
                    Err(err) => Ok(ApiResponse::error(err)),
                },
                Err(e) => Ok(ApiResponse::error(Box::new(e))),
            },
            Err(_) => Ok(ApiResponse::new(4001, "Session Not Found".to_owned(), Data::default())),
        }
    }
}
