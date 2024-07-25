use rocket::fs::TempFile;
use rocket::State;
use crab_rocket_schema::{DbPool};
use crate::mappers::file_mapper::{fetch_all_files, insert_files, retrieve_file_url_by_uuid};
use crate::models::file::File;
use crate::services::file_service::FileService;

impl FileService {
    pub async fn insert_file(
        pool: &State<DbPool>,
        files: Vec<TempFile<'_>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        match insert_files(pool, files).await {
            Ok(paths) => Ok(paths),
            Err(err) => {
                println!("{:?}", err.to_string());
                Err(Box::new(err))
            }
        }
    }
    pub async fn insert_avatar_file(
        pool: &State<DbPool>,
        file: rocket::fs::TempFile<'_>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match insert_files(pool, vec![file]).await {
            Ok(paths) => Ok(paths.first().unwrap().clone()),
            Err(err) => {
                println!("{:?}", err.to_string());
                Err(Box::new(err))
            }
        }
    }

    pub fn retrieve_file_url_by_uuid(
        pool: &State<DbPool>,
        uuid: uuid::Uuid,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match retrieve_file_url_by_uuid(pool, uuid) {
            Ok(url) => Ok(url),
            Err(err) => {
                println!("{:?}", err.to_string());
                Err(Box::new(err))
            }
        }
    }

    pub fn get_all_files(pool: &State<DbPool>,
    ) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        match fetch_all_files(pool) {
            Ok(all_files) => Ok(all_files),
            Err(err) => Err(Box::new(err)),
        }
    }
}