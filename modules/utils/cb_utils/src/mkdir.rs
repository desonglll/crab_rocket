use std::{fs, path::Path};

use colored::Colorize;

/// 在指定路径创建目录
/// 打印结果
///
/// ## 参数
///
/// * `path` - 要创建目录的路径。
///
/// ## 返回
///
/// 返回目录路径的字符串表示，如果失败则返回空字符串。
///
/// ```
pub fn make_directory(path: &str) -> () {
    match fs::create_dir(Path::new(path)) {
        Ok(_) => {
            println!("{}{}{}", "Created `/{".green(), path.green(), "}` successfully.".green());
        }
        Err(err) => {
            println!("{}{}{}", "Failed to create `/{".yellow(), path.yellow(), "}`.".yellow());
            println!("{}", err.to_string().yellow());
        }
    }
}

#[test]
fn test_make_directory() {
    let path = "upload";
    make_directory(path);
}
