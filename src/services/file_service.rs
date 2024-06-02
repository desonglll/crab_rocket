use rocket::fs::TempFile;
use uuid::Uuid;

pub trait GetFile {
    fn insert_file(
        files: Vec<TempFile>,
    ) -> impl std::future::Future<
        Output = Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>>,
    > + Send;
    fn retrieve_file_url_by_uuid(
        uuid: Uuid,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
