use std::path::Path;

use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
// use rocket::http::uri::Absolute;
use rocket::post;

// const HOST: Absolute<'static> = uri!("http://localhost:8000");
#[derive(FromForm)]
pub struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/upload", data = "<upload>")]
pub async fn upload(upload: Form<Upload<'_>>) -> std::io::Result<String> {
    let upload_data = upload.into_inner();
    if upload_data.save {
        // 如果 save 字段为 true，保存文件
        // 在这里处理上传的文件，例如将其保存到磁盘或进行其他操作
        // 例如：
        let mut file = upload_data.file;
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        let file_name = file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().to_string();
        let file_path = Path::new(root).join(file_name.clone());
        file.persist_to(file_path.clone()).await?;

        unimplemented!();
        Ok(String::from(
            Path::new("http://localhost:8000/retrieve/").join(file_name.clone()).to_str().unwrap(),
        ))
    } else {
        Ok(String::new())
    }
}
