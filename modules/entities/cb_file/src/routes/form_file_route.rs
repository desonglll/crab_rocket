use std::path::Path;

use mime_guess::mime;
use obj_traits::request::request_param::RequestParam;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::stream::ByteStream;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{get, options, post, State};
use serde_json::json;
use uuid::Uuid;

use crab_rocket_schema::DbPool;

use crate::controllers::file_controller::{
    self, download_file_controller, retrieve_file_controller,
};
use crate::mappers::file_mapper::retrieve_file_url_by_uuid;
use crate::models::file::File;
use crate::models::file_filter::FileFilter;
use crate::models::file_response::{FileDownloadResponse, FileRetrieveResponse};
use crate::models::upload::{AvatarUpload, Upload};

#[post("/upload", data = "<upload>")]
pub async fn upload(pool: &State<DbPool>, upload: Form<Upload<'_>>) -> Json<serde_json::Value> {
    println!("{:?}", upload.file);
    let upload_data = upload.into_inner();
    // 如果 save 字段为 true，保存文件
    // 在这里处理上传的文件，例如将其保存到磁盘或进行其他操作
    // 例如：
    let resp = file_controller::insert_file_controller(pool, upload_data.file).await.unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/upload")]
pub fn options_upload() -> Status {
    Status::Ok
}

#[get("/download/<uuid>")]
pub async fn download_file(pool: &State<DbPool>, uuid: Uuid) -> Option<FileDownloadResponse> {
    println!("{:?}", uuid);
    download_file_controller(pool, uuid).await
}

#[get("/retrieve/<uuid>")]
pub async fn retrieve_file(pool: &State<DbPool>, uuid: Uuid) -> Option<FileRetrieveResponse> {
    println!("{:?}", uuid);
    retrieve_file_controller(pool, uuid).await
}

#[post("/avatar_upload", data = "<upload>")]
pub async fn upload_avatar(
    pool: &State<DbPool>,
    upload: Form<AvatarUpload<'_>>,
) -> Json<serde_json::Value> {
    let upload_data = upload.into_inner();

    // 验证上传的文件是否为图片类型
    let mime_type: mime::Mime = mime_guess::from_path(Path::new(
        upload_data.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str(),
    ))
    .first_or_octet_stream(); // 假设有文件名可以检查
    println!("{:?}", upload_data.file.raw_name().unwrap().as_str());
    println!(
        "{:?}",
        upload_data.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str()
    );
    println!("{}", mime_type);

    if !mime_type.type_().eq(&mime::IMAGE) {
        return Json(json!({
            "code": 400,
            "message": "Uploaded file is not an image.",
            "data": null
        }));
    }

    let resp =
        file_controller::insert_avatar_file_controller(pool, upload_data.file).await.unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/files", data = "<param>")]
pub fn get_all_files(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<File, FileFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    let resp = file_controller::get_all_files_controller(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

/// ## 字节流下载文件
#[get("/byte/stream/<uuid>")]
pub async fn file_stream(pool: &State<DbPool>, uuid: Uuid) -> Option<ByteStream![Vec<u8>]> {
    match retrieve_file_url_by_uuid(&pool, uuid) {
        Ok(path) => {
            let mut file = match rocket::tokio::fs::File::open(path).await {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to open file: {:?}", e);
                    return None;
                }
            };
            let mut buffer = vec![0; 1024];

            Some(ByteStream! {
                loop {
                    let n = file.read(&mut buffer).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    yield buffer[..n].to_vec();
                }
            })
        }
        Err(e) => {
            println!("{:?}", e.to_string());
            None
        }
    }
}
