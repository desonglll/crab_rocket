use crate::{
    mappers::file_mapper::{fetch_all_files, insert_files, retrieve_file_url_by_uuid},
    models::file::File,
    services::file_service::GetFile,
};
use crab_rocket_schema::{establish_pg_connection, DbPool};

impl GetFile for File {
    async fn insert_file(
        pool: &DbPool,
        files: Vec<rocket::fs::TempFile<'_>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match insert_files(&mut conn, files).await {
                Ok(paths) => Ok(paths),
                Err(err) => {
                    println!("{:?}", err.to_string());
                    Err(Box::new(err))
                }
            },
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }
    async fn insert_avatar_file(
        pool: &DbPool,
        file: rocket::fs::TempFile<'_>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match insert_files(&mut conn, vec![file]).await {
                Ok(paths) => Ok(paths.first().unwrap().clone()),
                Err(err) => {
                    println!("{:?}", err.to_string());
                    Err(Box::new(err))
                }
            },
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn retrieve_file_url_by_uuid(
        pool: &DbPool,
        uuid: uuid::Uuid,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match retrieve_file_url_by_uuid(&mut conn, uuid) {
                Ok(url) => Ok(url),
                Err(err) => {
                    println!("{:?}", err.to_string());
                    Err(Box::new(err))
                }
            },
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn get_all_files(pool: &DbPool) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        match establish_pg_connection(pool) {
            Ok(mut conn) => match fetch_all_files(&mut conn) {
                Ok(all_files) => Ok(all_files),
                Err(err) => Err(Box::new(err)),
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
