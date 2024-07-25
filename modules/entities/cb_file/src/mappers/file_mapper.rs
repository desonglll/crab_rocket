use std::path::Path;

use diesel::{RunQueryDsl};
use diesel::prelude::*;
use rocket::fs::TempFile;
use rocket::State;
use uuid::Uuid;
use crab_rocket_schema::{DbPool, establish_pg_connection};

use crab_rocket_schema::schema::file_table::{self};
use crab_rocket_schema::schema::file_table::dsl::*;

use crate::models::file::{File, PostFile};

pub async fn insert_files(
    pool: &State<DbPool>,
    files: Vec<TempFile<'_>>,
) -> Result<Vec<String>, std::io::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
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
        let new_file = PostFile {
            file_id: file_uuid,
            file_name: &original_file_name,
            file_url: file_path.to_str().unwrap(),
        };
        let result = diesel::insert_into(file_table).values(new_file).execute(&mut conn);
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
    pool: &State<DbPool>, uuid: Uuid,
) -> Result<String, diesel::result::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
    println!("enter mapper: {:?}", uuid);
    file_table.filter(file_table::file_id.eq(uuid)).select(file_url).first::<String>(&mut conn)
}

pub fn fetch_all_files(pool: &State<DbPool>,
) -> Result<Vec<File>, diesel::result::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
    let result = file_table.order(file_table::uploaded_at.desc()).load::<File>(&mut conn)?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crab_rocket_schema::{DbPool, establish_pool};

    use crate::mappers::file_mapper::fetch_all_files;

    #[test]
    fn test_fetch_all_files() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        let all_files = fetch_all_files(pool);
        println!("{all_files:?}");
    }
}
