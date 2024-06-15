pub mod mkdir;
pub mod time;
pub fn run_preload() {
    match mkdir::make_directory("upload") {
        Ok(_) => {
            println!("Created upload successfully");
        }
        Err(e) => {
            if e.to_string() == "File exists (os error 17)" {
                println!("Directory `upload` already exists.");
            } else {
                println!("{:?}", e.to_string());
            }
        }
    }
}
