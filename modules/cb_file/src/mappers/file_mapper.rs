use crate::models::file::{File, NewFile};
use crab_rocket_schema::schema::file_table::dsl::*;
use crab_rocket_schema::schema::file_table::{self};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use rocket::fs::TempFile;
use std::path::Path;
use uuid::Uuid;

pub async fn insert_files(
    conn: &mut PgConnection,
    files: Vec<TempFile<'_>>,
) -> Result<Vec<String>, std::io::Error> {
    let mut result_paths: Vec<String> = Vec::new();
    for mut f in files {
        // 如果 base_dir 为 None，则使用默认值
        //遍历文件列表
        let key_value = std::env::var("CRAB_ROCKET_WORKING_DIR").unwrap();
        let upload_folder = format!("{}/upload", key_value);

        println!("upload_folder: {:?}", upload_folder);
        let original_file_name =
            f.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().to_string();
        println!("file_name: {:?}", file_name);
        let file_uuid = Uuid::new_v4();
        println!("file_uuid: {:?}", file_uuid.to_string());
        let file_path = Path::new(upload_folder.as_str()).join(original_file_name.clone());
        println!("file_path: {:?}", file_path.clone().to_str());
        let new_file = NewFile {
            id: file_uuid,
            file_name: &original_file_name,
            file_url: file_path.to_str().unwrap(),
        };
        let result = diesel::insert_into(file_table).values(new_file).execute(conn);
        println!("{:?}", result);
        println!("---");
        // 在Linux系统上，不同的文件系统或挂载点之间无法直接进行文件移动操作 (rename)，因此会产生这个错误
        // 这里我们使用 copy_to 代替 persist_to
        // f.persist_to(file_path.clone()).await?;
        println!("{:?}", file_path.clone().to_str());
        f.copy_to(file_path.clone()).await?;
        print!("---");
        result_paths.push(file_uuid.to_string());
    }
    Ok(result_paths)
}

pub fn retrieve_file_url_by_uuid(
    conn: &mut PgConnection,
    uuid: Uuid,
) -> Result<String, diesel::result::Error> {
    println!("enter mapper: {:?}", uuid);
    file_table.filter(file_table::id.eq(uuid)).select(file_url).first::<String>(conn)
}

pub fn fetch_all_files(conn: &mut PgConnection) -> Result<Vec<File>, diesel::result::Error> {
    file_table.order(file_table::uploaded_at.desc()).load::<File>(conn)
}

#[cfg(test)]
mod test {
    use crate::mappers::file_mapper::fetch_all_files;
    use crab_rocket_schema::establish_pg_connection;
    #[test]
    fn test_fetch_all_files() {
        match establish_pg_connection() {
            Ok(mut conn) => {
                let all_files = fetch_all_files(&mut conn);
                println!("{all_files:?}");
            }
            Err(_) => (),
        }
    }
}
