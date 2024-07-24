use rocket::fs::TempFile;
use uuid::Uuid;

use crab_rocket_schema::DbPool;

use crate::models::file::File;

pub trait GetFile {
    fn insert_file(
        pool: &DbPool,
        files: Vec<TempFile<'_>>,
    ) -> impl std::future::Future<
        Output = Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>>,
    > + Send;
    fn insert_avatar_file(
        pool: &DbPool,
        file: TempFile<'_>,
    ) -> impl std::future::Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send;
    fn retrieve_file_url_by_uuid(
        pool: &DbPool,
        uuid: Uuid,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    fn get_all_files(pool: &DbPool) -> Result<Vec<File>, Box<dyn std::error::Error>>;
}
