pub mod mkdir;
pub fn run_preload() {
    println!("{:?}", mkdir::make_directory("upload"));
}
