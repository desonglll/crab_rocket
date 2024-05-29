use std::{fs, path::Path};

/// Creates a directory at a specified path.
/// Returns `Ok(())` if the directory was created successfully, or an error if not.
pub fn make_directory(path: &str) -> Result<String, std::io::Error> {
    match fs::create_dir(Path::new(path)) {
        Ok(_) => Ok(String::from(path)),
        Err(err) => {
            if err.to_string() == "File exists (os error 17)" {
                // println!("Delete exists directory {:?}", path);
                // fs::remove_dir_all(path)?;
                // fs::create_dir(Path::new(path))?;
                // Ok(String::from(path))
                Err(err)
            } else {
                Err(err)
            }
        }
    }
}

#[test]
fn test_make_directory() {
    let path = "upload";
    match make_directory(path) {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(e) => {
            println!("{:?}", e.to_string());
        }
    }
}
