use std::path::Path;

use rocket::{
    fs::TempFile,
    http::{ContentType, MediaType},
    State,
};
use uuid::Uuid;

use crab_rocket_schema::DbPool;
use obj_traits::response::data::Data;
use obj_traits::{request::request_param::RequestParam, response::api_response::ApiResponse};

use crate::models::{
    self,
    file::File,
    file_filter::FileFilter,
    file_response::{FileDownloadResponse, FileRetrieveResponse},
};
use crate::services::file_service::FileService;

pub fn get_all_files_controller(
    pool: &State<DbPool>,
    param: &RequestParam<File, FileFilter>,
) -> Result<ApiResponse<Data<Vec<File>>>, Box<dyn std::error::Error>> {
    match FileService::get_all_files(pool, param) {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Ok(ApiResponse::error(e)),
    }
}

pub async fn insert_file_controller(
    pool: &State<DbPool>,
    files: Vec<TempFile<'_>>,
) -> Result<ApiResponse<Data<Vec<File>>>, Box<dyn std::error::Error>> {
    match FileService::insert_files(pool, files).await {
        Ok(result) => Ok(ApiResponse::success(result)),
        Err(e) => Ok(ApiResponse::error(e)),
    }
}

pub async fn insert_avatar_file_controller(
    pool: &State<DbPool>,
    file: TempFile<'_>,
) -> Result<ApiResponse<Data<File>>, Box<dyn std::error::Error>> {
    match FileService::insert_single_file(pool, file).await {
        Ok(result) => Ok(ApiResponse::success(result)),
        Err(e) => Ok(ApiResponse::error(e)),
    }
}

pub async fn retrieve_file_controller(
    pool: &State<DbPool>,
    uuid: Uuid,
) -> Option<FileRetrieveResponse> {
    match FileService::retrieve_file_url_by_uuid(pool, uuid) {
        Ok(path) => {
            println!("File Path: {}", path);
            let named_file = rocket::fs::NamedFile::open(&path).await.ok()?;
            let file_name = Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("downloaded_file")
                .to_string();
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream();
            let media_type =
                mime_type.essence_str().parse::<MediaType>().unwrap_or(MediaType::Binary);

            Some(FileRetrieveResponse {
                file: named_file,
                file_name,
                content_type: ContentType(media_type),
            })
        }
        Err(e) => {
            println!("{:?}", e.to_string());
            None
        }
    }
}

pub async fn download_file_controller(
    pool: &State<DbPool>,
    uuid: Uuid,
) -> Option<models::file_response::FileDownloadResponse> {
    match FileService::retrieve_file_url_by_uuid(pool, uuid) {
        Ok(path) => {
            println!("File Path: {}", path);
            let file = rocket::fs::NamedFile::open(path.clone()).await.ok();
            let file_name = Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("downloaded_file")
                .to_string();
            let mime_type = mime_guess::from_path(&path).first_or_octet_stream();
            let media_type =
                mime_type.essence_str().parse::<MediaType>().unwrap_or(MediaType::Binary);

            Some(FileDownloadResponse {
                file: file.unwrap(),
                file_name,
                content_type: ContentType(media_type),
            })
        }
        Err(e) => {
            println!("{:?}", e.to_string());
            None
        }
    }
}
// pub async fn retrieve_file_controller(uuid: Uuid) -> _ {

// }
