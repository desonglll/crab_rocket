use std::error::Error;
use crab_rocket_schema::establish_pg_connection;
use obj_traits::mapper::mapper_crud::MapperCRUD;
use obj_traits::request::pagination_request_param::PaginationParam;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use obj_traits::service::service_crud::ServiceCRUD;
use crate::mappers::role_mapper::RoleMapper;
use crate::models::role::{NewRole, PatchRole, Role};

pub struct RoleService {}

impl ServiceCRUD<Role, NewRole, PatchRole, RequestParam<PaginationParam>> for RoleService {
    fn get_all(param: &RequestParam<PaginationParam>) -> Result<Data<Vec<Role>>, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match RoleMapper::get_all(&mut conn, param) {
                    Ok(all_roles) => Ok(all_roles),
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

    fn get_by_id(pid: i32) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match RoleMapper::get_by_id(&mut conn, pid) {
                    Ok(role) => Ok(role),
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

    fn add_single(obj: &NewRole) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match RoleMapper::add_single(&mut conn, obj) {
                    Ok(inserted_role) => Ok(inserted_role),
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

    fn delete_by_id(pid: i32) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match RoleMapper::delete_by_id(&mut conn, pid) {
                    Ok(deleted_role) => Ok(deleted_role),
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

    fn update_by_id(pid: i32, obj: &PatchRole) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match RoleMapper::update_by_id(&mut conn, pid, obj) {
                    Ok(updated_role) => Ok(updated_role),
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