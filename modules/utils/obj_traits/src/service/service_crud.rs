use std::error::Error;

use rocket::State;

use crab_rocket_schema::{DbPool, establish_pg_connection};

use crate::{
    mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam, response::data::Data,
};

/// ## Construct
/// T is for the fully fields object.
///
/// U is for the new added object, typically for no id.
///
/// V is for the updated object, typically for no id.
pub trait ServiceCRUD {
    type Item;
    type PostItem;
    type PatchItem;
    type Param;
    fn get_all(
        pool: &State<DbPool>,
        param: &Self::Param,
    ) -> Result<Data<Vec<Self::Item>>, Box<dyn std::error::Error>>;
    fn get_by_id(pool: &State<DbPool>, pid: i32) -> Result<Self::Item, Box<dyn std::error::Error>>;
    fn add_single(
        pool: &State<DbPool>,
        obj: &Self::PostItem,
    ) -> Result<Self::Item, Box<dyn std::error::Error>>;
    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Self::Item, Box<dyn std::error::Error>>;
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &Self::PatchItem,
    ) -> Result<Self::Item, Box<dyn std::error::Error>>;
    fn filter(
        pool: &State<DbPool>,
        param: &Self::Param,
    ) -> Result<Data<Vec<Self::Item>>, Box<dyn std::error::Error>>;
}
pub fn service_get_all<Obj, ObjMapper, ObjFilter>(
    pool: &State<DbPool>,
    param: &RequestParam<ObjFilter>,
) -> Result<Data<Vec<Obj>>, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj, Param = RequestParam<ObjFilter>>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::get_all(&mut conn, param) {
            Ok(data) => Ok(data),
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

pub fn service_get_by_id<Obj, ObjMapper>(
    pool: &State<DbPool>,
    pid: i32,
) -> Result<Obj, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::get_by_id(&mut conn, pid) {
            Ok(data) => Ok(data),
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

pub fn service_add_single<Obj, ObjMapper, NewObj>(
    pool: &State<DbPool>,
    obj: &NewObj,
) -> Result<Obj, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj, PostItem = NewObj>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::add_single(&mut conn, obj) {
            Ok(employee) => Ok(employee),
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

pub fn service_delete_by_id<Obj, ObjMapper>(
    pool: &State<DbPool>,
    pid: i32,
) -> Result<Obj, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::delete_by_id(&mut conn, pid) {
            Ok(data) => Ok(data),
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
pub fn service_update_by_id<Obj, ObjMapper, PatchObj>(
    pool: &State<DbPool>,
    pid: i32,
    obj: &PatchObj,
) -> Result<Obj, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj, PatchItem = PatchObj>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::update_by_id(&mut conn, pid, obj) {
            Ok(data) => Ok(data),
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
pub fn service_filter<Obj, ObjMapper, ObjFilter>(
    pool: &State<DbPool>,
    param: &RequestParam<ObjFilter>,
) -> Result<Data<Vec<Obj>>, Box<dyn Error>>
where
    ObjMapper: MapperCRUD<Item = Obj, Param = RequestParam<ObjFilter>>,
{
    match establish_pg_connection(pool) {
        Ok(mut conn) => match ObjMapper::get_all(&mut conn, param) {
            Ok(data) => Ok(data),
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
