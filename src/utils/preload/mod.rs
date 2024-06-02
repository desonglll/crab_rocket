pub mod mkdir;
pub fn run_preload() {
    println!("Created {:?} folder successfully!", mkdir::make_directory("upload").unwrap());
}
