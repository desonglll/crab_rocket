use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use rand::Rng;
use rocket::{Data, fs::NamedFile, get, post};
use rocket::data::ToByteUnit;
use rocket::http::uri::Absolute;
use rocket::request::FromParam;
use rocket::uri;
use rocket::UriDisplayPath;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(&self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(PasteId(Cow::Borrowed(param)))
        } else {
            Err(param)
        }
    }
}
#[get("/static_file/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
#[get("/retrieve_bin/<id>")]
pub async fn retrieve_bin(id: PasteId<'_>) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(id.file_path()).await.ok()
}
#[post("/upload_bin", data = "<paste>")]
pub async fn upload_bin(paste: Data<'_>) -> std::io::Result<String> {
    // println!("{:?}", paste);
    let id = PasteId::new(3);
    let path = id.file_path();
    paste.open(128.megabytes()).into_file(path).await?; //设置文件大小限制
    Ok(uri!(HOST, retrieve_bin(id)).to_string())
}
