use std::fs;
use std::path::PathBuf;

use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::post;

#[derive(FromForm, Debug)]
pub struct Upload<'r> {
    pub save: bool,
    pub file: TempFile<'r>,
}

#[post("/upload", data = "<upload>")]
pub fn single_upload(upload: Form<Upload<'_>>) {
    let upload = upload.into_inner();

    if upload.save {
        // 指定您希望保存文件的文件夹路径
        let save_folder = "./files/";

        // 检查文件夹是否存在
        if !folder_exists(&save_folder) {
            // 如果文件夹不存在，则创建它
            if let Err(err) = fs::create_dir_all(&save_folder) {
                panic!("Failed to create folder: {:?}", err);
            }
        }

        // 生成保存文件的路径
        let mut save_path = PathBuf::from(save_folder);

        // 获取上传文件的原始文件名
        let file_name = upload.file.raw_name().unwrap();

        // 将文件名追加到保存文件夹路径中
        save_path.push(file_name.dangerous_unsafe_unsanitized_raw().as_str());

        // 尝试将临时文件移动到指定路径
        fs::rename(upload.file.path().unwrap(), save_path).expect("Failed to save file");
    } else {
        // 如果不保存文件，您可以选择删除临时文件
        std::fs::remove_file(upload.file.path().unwrap()).expect("Failed to remove file");
    }
}
// 检查文件夹是否存在
fn folder_exists(path: &str) -> bool { fs::metadata(path).is_ok() }
