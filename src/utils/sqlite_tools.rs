use std::fs;

use rusqlite::{Connection, Result};
#[allow(dead_code)]
const CREATE_TABLE_STATEMENT: &str = "CREATE TABLE IF NOT EXISTS tasks(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT 
)";

pub fn create_table(conn: &Connection, statement: &str) -> Result<()> {
    conn.execute(
        &statement,
        (), // empty list of parameters.
    )?;
    Ok(())
}

pub fn init_sqlite_db() -> Result<()> {
    let file_path = "tasks.sqlite";
    // 检查文件是否存在
    if fs::metadata(&file_path).is_ok() {
        // 文件存在，尝试删除文件
        if let Err(err) = fs::remove_file(&file_path) {
            eprintln!("Failed to delete file: {}", err);
        } else {
            println!("File {} deleted successfully", file_path);
        }
    } else {
        // 文件不存在，打印消息
        println!("File {} does not exist", file_path);
    }
    let conn = Connection::open(file_path)?;
    let _ = create_table(&conn, CREATE_TABLE_STATEMENT);
    Ok(())
}
