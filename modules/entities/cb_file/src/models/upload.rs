use rocket::{FromForm, fs::TempFile, serde::Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResponseData {
    pub paths: Vec<String>,
}
#[derive(FromForm)]
pub struct Upload<'r> {
    pub save: bool,
    pub file: Vec<TempFile<'r>>,
}
#[derive(FromForm)]
pub struct AvatarUpload<'r> {
    pub save: bool,
    pub file: TempFile<'r>,
}
