use crate::mappers::file_mapper::{fetch_all_files, insert_files, retrieve_file_url_by_uuid};
use crate::models::file::File;
use crate::models::file_filter::FileFilter;
use crate::services::file_service::FileService;
use crab_rocket_schema::DbPool;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use rocket::fs::TempFile;
use rocket::State;

impl FileService {
    pub async fn insert_files(
        pool: &State<DbPool>,
        files: Vec<TempFile<'_>>,
    ) -> Result<Data<Vec<File>>, Box<dyn std::error::Error + Send + Sync>> {
        match insert_files(pool, files).await {
            Ok(data) => Ok(Data::new(data, None)),
            Err(err) => {
                println!("{:?}", err.to_string());
                Err(Box::new(err))
            }
        }
    }
    pub async fn insert_single_file(
        pool: &State<DbPool>,
        file: rocket::fs::TempFile<'_>,
    ) -> Result<Data<File>, Box<dyn std::error::Error + Send + Sync>> {
        match insert_files(pool, vec![file]).await {
            Ok(data) => Ok(Data::new(data[0].clone(), None)),
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

    pub fn get_all_files(
        pool: &State<DbPool>,
        param: &RequestParam<File, FileFilter>,
    ) -> Result<Data<Vec<File>>, Box<dyn std::error::Error>> {
        match fetch_all_files(pool, param) {
            Ok(all_files) => Ok(all_files),
            Err(err) => Err(Box::new(err)),
        }
    }
}
