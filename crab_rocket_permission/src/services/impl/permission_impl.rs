use crate::{
    mappers::permission_mapper::fetch_all_permissions, models::permission::Permission,
    services::permission_service::GetPermission,
};
use crab_rocket_schema::establish_pg_connection;
impl GetPermission for Permission {
    fn get_all_permissions() -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_all_permissions(&mut conn) {
                    Ok(all_permissions) => Ok(all_permissions),
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
