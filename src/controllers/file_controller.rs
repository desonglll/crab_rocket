use rocket::fs::TempFile;
use uuid::Uuid;

use crate::{models::files::File, services::file_service::GetFile};

pub async fn insert_file_controller(files: Vec<TempFile<'_>>) -> (i32, String, Vec<String>) {
    match File::insert_file(files).await {
        Ok(result) => (200, String::from("INSERT FILES OK"), result),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

pub async fn retrieve_file_controller(uuid: Uuid) -> Option<rocket::fs::NamedFile> {
    match File::retrieve_file_url_by_uuid(uuid) {
        Ok(path) => rocket::fs::NamedFile::open(path).await.ok(),
        Err(e) => {
            println!("{:?}", e.to_string());
            None
        }
    }
}
