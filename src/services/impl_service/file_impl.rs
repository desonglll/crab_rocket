use crate::{
    establish_pg_connection,
    mappers::file_mapper::{fetch_all_files, insert_files, retrieve_file_url_by_uuid},
    models::files::File,
    services::file_service::GetFile,
};

impl GetFile for File {
    async fn insert_file(
        files: Vec<rocket::fs::TempFile<'_>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match insert_files(&mut conn, files).await {
                    Ok(paths) => Ok(paths),
                    Err(err) => {
                        println!("{:?}", err.to_string());
                        Err(Box::new(err))
                    }
                }
            }
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn retrieve_file_url_by_uuid(
        uuid: uuid::Uuid,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match retrieve_file_url_by_uuid(&mut conn, uuid) {
                    Ok(url) => Ok(url),
                    Err(err) => {
                        println!("{:?}", err.to_string());
                        Err(Box::new(err))
                    }
                }
            }
            Err(e) => {
                println!("Error");
                Err(Box::new(e))
            }
        }
    }

    fn get_all_files() -> Result<Vec<File>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_all_files(&mut conn) {
                    Ok(all_files) => Ok(all_files),
                    Err(err) => Err(Box::new(err)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
