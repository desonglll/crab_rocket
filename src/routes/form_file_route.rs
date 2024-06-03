use crate::controllers::file_controller::{self, retrieve_file_controller};
use crate::models::upload::Upload;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
use uuid::Uuid;

#[post("/upload", data = "<upload>")]
pub async fn upload(upload: Form<Upload<'_>>) -> Json<serde_json::Value> {
    println!("{:?}", upload.file);
    let upload_data = upload.into_inner();
    // 如果 save 字段为 true，保存文件
    // 在这里处理上传的文件，例如将其保存到磁盘或进行其他操作
    // 例如：
    let (code, message, paths) = file_controller::insert_file_controller(upload_data.file).await;
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":paths
    }))
    .unwrap();
    Json(response)
}
#[get("/retrieve/<uuid>")]
pub async fn retrieve(uuid: Uuid) -> Option<rocket::fs::NamedFile> {
    println!("{:?}", uuid);
    retrieve_file_controller(uuid).await
}

#[get("/files")]
pub fn get_all_files() -> Json<serde_json::Value> {
    let (code, message, result) = file_controller::get_all_files_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":result
    }))
    .unwrap();
    Json(response)
}
