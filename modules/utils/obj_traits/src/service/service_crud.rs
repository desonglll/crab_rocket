use crab_rocket_schema::DbPool;
use rocket::State;

use crate::{
    mapper::mapper_crud::MapperCRUD, request::request_param::RequestParam, response::data::Data,
};

pub trait ServiceCRUD {
    type Item: Default;
    type PostItem;
    type PatchItem;
    type Filter;
    type Mapper: MapperCRUD<
        Item = Self::Item,
        PostItem = Self::PostItem,
        PatchItem = Self::PatchItem,
        Filter = Self::Filter,
    >;
    fn get_all(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Self::Item>>, Box<dyn std::error::Error>> {
        match Self::Mapper::get_all(pool, param) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
    fn get_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Self::Item>, Box<dyn std::error::Error>> {
        match Self::Mapper::get_by_id(pool, pid) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
    fn add_single(
        pool: &State<DbPool>,
        obj: &Self::PostItem,
    ) -> Result<Data<Self::Item>, Box<dyn std::error::Error>> {
        match Self::Mapper::add_single(pool, obj) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
    fn delete_by_id(
        pool: &State<DbPool>,
        pid: i32,
    ) -> Result<Data<Self::Item>, Box<dyn std::error::Error>> {
        match Self::Mapper::delete_by_id(pool, pid) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
    fn update_by_id(
        pool: &State<DbPool>,
        pid: i32,
        obj: &Self::PatchItem,
    ) -> Result<Data<Self::Item>, Box<dyn std::error::Error>> {
        match Self::Mapper::update_by_id(pool, pid, obj) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
    fn filter(
        pool: &State<DbPool>,
        param: &RequestParam<Self::Item, Self::Filter>,
    ) -> Result<Data<Vec<Self::Item>>, Box<dyn std::error::Error>> {
        match Self::Mapper::filter(pool, param) {
            Ok(data) => Ok(data),
            Err(err) => Err(Box::new(err)),
        }
    }
}
