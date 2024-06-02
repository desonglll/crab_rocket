use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, post};
use std::path::Path;
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseData {
    paths: Vec<String>,
}
#[derive(FromForm)]
pub struct Upload<'r> {
    save: bool,
    file: Vec<TempFile<'r>>,
}
#[post("/upload", data = "<upload>")]
pub async fn upload(upload: Form<Upload<'_>>) -> Result<Json<ResponseData>, std::io::Error> {
    println!("{:?}", upload.file);
    let upload_data = upload.into_inner();
    if upload_data.save {
        // 如果 save 字段为 true，保存文件
        // 在这里处理上传的文件，例如将其保存到磁盘或进行其他操作
        // 例如：

        let file = upload_data.file;
        let mut paths = Vec::new();
        for mut f in file {
            let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
            println!("root: {:?}", root);
            let file_name = f.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().to_string();
            println!("filename: {:?}", file_name);
            let file_path = Path::new(root).join(file_name.clone());
            println!("file_path: {:?}", file_path.clone().to_str());
            // 在Linux系统上，不同的文件系统或挂载点之间无法直接进行文件移动操作 (rename)，因此会产生这个错误
            // 这里我们使用 copy_to 代替 persist_to
            // f.persist_to(file_path.clone()).await?;
            f.copy_to(file_path.clone()).await?;
            paths.push(String::from(file_name))
        }
        Ok(Json(ResponseData {
            paths,
        }))
    } else {
        Ok(Json(ResponseData {
            paths: Vec::new(),
        }))
    }
}
#[get("/retrieve/<path>")]
pub async fn retrieve(path: &str) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(format!(
        "{}/{}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload"),
        path
    ))
    .await
    .ok()
}
