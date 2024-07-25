use std::path::Path;

use rocket::{
    fs::TempFile,
    http::{ContentType, MediaType},
    State,
};
use uuid::Uuid;

use crab_rocket_schema::DbPool;

use crate::{
    models::{
        self,
        file::File,
        file_response::{FileDownloadResponse, FileRetrieveResponse},
    },
};
use crate::services::file_service::FileService;

pub async fn insert_file_controller(
    pool: &State<DbPool>,
    files: Vec<TempFile<'_>>,
) -> (i32, String, Vec<String>) {
    match FileService::insert_file(pool, files).await {
        Ok(result) => (200, String::from("INSERT FILES OK"), result),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

pub async fn insert_avatar_file_controller(
    pool: &State<DbPool>,
    file: TempFile<'_>,
) -> (i32, String, String) {
    match FileService::insert_avatar_file(pool, file).await {
        Ok(result) => (200, String::from("INSERT FILES OK"), result),
        Err(e) => (204, e.to_string(), String::new()),
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

pub fn get_all_files_controller(pool: &State<DbPool>) -> (i32, String, Vec<File>) {
    match FileService::get_all_files(pool) {
        Ok(all_files) => (200, String::from("GET ALL FILES OK"), all_files),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}
