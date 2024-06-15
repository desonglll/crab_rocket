use crate::mappers::role_mapper::{delete_role_by_id, fetch_role_by_id, update_role_by_id};
use crate::models::role::PatchRole;
use crate::{
    mappers::role_mapper::{fetch_all_roles, insert_role},
    models::role::Role,
    services::role_service::GetRole,
};
use crab_rocket_schema::establish_pg_connection;
use std::error::Error;

impl GetRole for Role {
    fn insert_role(
        role: &crate::models::role::NewRole,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match insert_role(&mut conn, &role) {
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

    fn get_all_roles() -> Result<Vec<Role>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_all_roles(&mut conn) {
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

    fn delete_role_by_id(id: i32) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match delete_role_by_id(&mut conn, id) {
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

    fn get_role_by_id(id: i32) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_role_by_id(&mut conn, id) {
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

    fn update_role_by_id(id: i32, role: &PatchRole) -> Result<Role, Box<dyn Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match update_role_by_id(&mut conn, id, role) {
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
