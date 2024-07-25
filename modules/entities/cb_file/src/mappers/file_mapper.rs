use std::path::Path;

use crab_rocket_schema::{establish_pg_connection, DbPool};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use obj_traits::request::pagination_request_param::Pagination;
use obj_traits::request::request_param::RequestParam;
use obj_traits::response::data::Data;
use rocket::fs::TempFile;
use rocket::State;
use uuid::Uuid;

use crab_rocket_schema::schema::file_table::dsl::*;
use crab_rocket_schema::schema::file_table::{self};

use crate::models::file::{File, PostFile};
use crate::models::file_filter::FileFilter;

pub async fn insert_files(
    pool: &State<DbPool>,
    files: Vec<TempFile<'_>>,
) -> Result<Vec<File>, std::io::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
    let mut results: Vec<File> = Vec::new();
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

        let result: File = diesel::insert_into(file_table)
            .values(new_file)
            .returning(File::as_returning())
            .get_result(&mut conn)
            .unwrap();
        println!("{:?}", result);
        println!("---");
        // 在Linux系统上，不同的文件系统或挂载点之间无法直接进行文件移动操作 (rename)，因此会产生这个错误
        // 这里我们使用 copy_to 代替 persist_to
        // f.persist_to(file_path.clone()).await?;
        println!("{:?}", file_path.clone().to_str());
        f.copy_to(file_path.clone()).await?;
        print!("---");
        results.push(result)
    }
    Ok(results)
}

pub fn retrieve_file_url_by_uuid(
    pool: &State<DbPool>,
    uuid: Uuid,
) -> Result<String, diesel::result::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
    println!("enter mapper: {:?}", uuid);
    file_table.filter(file_table::file_id.eq(uuid)).select(file_url).first::<String>(&mut conn)
}

pub fn fetch_all_files(
    pool: &State<DbPool>,
    param: &RequestParam<File, FileFilter>,
) -> Result<Data<Vec<File>>, diesel::result::Error> {
    let mut conn = establish_pg_connection(pool).expect("error establishing connection.");
    // 计算分页相关
    let pagination = param.pagination.unwrap_or_default().clone();
    let page = (pagination.offset.unwrap() / pagination.limit.unwrap()) + 1;
    let per_page = pagination.limit.unwrap();
    // 获取总记录数
    let total_count = file_table.count().get_result::<i64>(&mut conn)? as i32;
    // 计算总页数
    let total_pages = (total_count + per_page - 1) / per_page;

    let previous_page_offset = (page - 2) * per_page;
    let next_page_offset = page * per_page;
    let pagination = Pagination::new(
        page,
        per_page,
        total_pages,
        total_count,
        Some(format!("?limit={}&offset={}", per_page, next_page_offset)),
        Some(format!("?limit={}&offset={}", per_page, previous_page_offset)),
    );

    let data = file_table.order(file_table::uploaded_at.desc()).load::<File>(&mut conn)?;
    let body = Data::new(data, Some(pagination));
    println!("Getting tasks successfully.");
    Ok(body)
}

#[cfg(test)]
mod test {
    use obj_traits::request::{
        pagination_request_param::{PaginationParam, PaginationParamTrait},
        request_param::RequestParam,
    };
    use rocket::State;

    use crab_rocket_schema::{establish_pool, DbPool};

    use crate::mappers::file_mapper::fetch_all_files;

    #[test]
    fn test_fetch_all_files() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);

        let param = RequestParam::new(Some(PaginationParam::demo()), None);
        let all_files = fetch_all_files(pool, &param);
        println!("{all_files:?}");
    }
}
