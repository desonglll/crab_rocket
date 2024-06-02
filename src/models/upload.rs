use rocket::{fs::TempFile, serde::Serialize, FromForm};

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
