use crate::{
    establish_pg_connection,
    mappers::role_mapper::{fetch_all_roles, insert_role},
    models::role::Role,
    services::role_service::GetRole,
};

impl GetRole for Role {
    fn insert_role(
        role: &crate::models::role::NewRole,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match insert_role(&mut conn, &role) {
                Ok(inserted_role) => Ok(inserted_role),
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
    fn get_all_roles() -> Result<Vec<Role>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_all_roles(&mut conn) {
                Ok(all_roles) => Ok(all_roles),
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
