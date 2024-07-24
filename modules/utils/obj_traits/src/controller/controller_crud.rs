use std::error::Error;
use std::fmt::Debug;

use rocket::State;

use crab_rocket_schema::DbPool;

use crate::request::request_param::RequestParam;
use crate::response::api_response::ApiResponse;
use crate::response::data::Data;
use crate::service::service_crud::ServiceCRUD;

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait ControllerCRUD {
    type Item;
    type PostItem;
    type PatchItem;
    type Param;
    fn get_all(
        pool: &State<DbPool>,
        param: &Self::Param,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>>;
    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn std::error::Error>>;
    fn add_single(
        pool: &State<DbPool>,
        obj: &mut Self::PostItem,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn std::error::Error>>;
    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn std::error::Error>>;
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &Self::PatchItem,
    ) -> Result<ApiResponse<Self::Item>, Box<dyn std::error::Error>>;
    fn filter(
        pool: &State<DbPool>,
        param: &Self::Param,
    ) -> Result<ApiResponse<Data<Vec<Self::Item>>>, Box<dyn std::error::Error>>;
}

pub fn controller_get_all<Obj, ObjService, ObjFilter>(
    pool: &State<DbPool>,
    param: &RequestParam<ObjFilter>,
) -> Result<ApiResponse<Data<Vec<Obj>>>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj, Param = RequestParam<ObjFilter>>,
{
    match ObjService::get_all(pool, param) {
        Ok(all_employees) => {
            let response = ApiResponse::success(all_employees);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}

pub fn controller_get_by_id<Obj, ObjService>(
    pool: &State<DbPool>,
    pid: i32,
) -> Result<ApiResponse<Obj>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj>,
    Obj: std::default::Default,
{
    match ObjService::get_by_id(pool, pid) {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}
pub fn controller_add_single<Obj, ObjService, NewObj>(
    pool: &State<DbPool>,
    obj: &mut NewObj,
) -> Result<ApiResponse<Obj>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj, PostItem = NewObj>,
    Obj: std::default::Default,
{
    match ObjService::add_single(pool, obj) {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}
pub fn controller_delete_by_id<Obj, ObjService>(
    pool: &State<DbPool>,
    pid: i32,
) -> Result<ApiResponse<Obj>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj>,
    Obj: std::default::Default,
{
    match ObjService::delete_by_id(pool, pid) {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}
pub fn controller_update_by_id<Obj, ObjService, PatchObj>(
    pool: &State<DbPool>,
    pid: i32,
    obj: &PatchObj,
) -> Result<ApiResponse<Obj>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj, PatchItem = PatchObj>,
    Obj: std::default::Default,
{
    match ObjService::update_by_id(pool, pid, obj) {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}
pub fn controller_filter<Obj, ObjService, ObjFilter>(
    pool: &State<DbPool>,
    param: &RequestParam<ObjFilter>,
) -> Result<ApiResponse<Data<Vec<Obj>>>, Box<dyn Error>>
where
    ObjService: ServiceCRUD<Item = Obj, Param = RequestParam<ObjFilter>>,
    ObjFilter: Debug,
{
    match ObjService::filter(pool, param) {
        Ok(all_employees) => {
            let response = ApiResponse::success(all_employees);
            Ok(response)
        }
        Err(e) => {
            println!("{e:?}");
            Ok(ApiResponse::error(e))
        }
    }
}
