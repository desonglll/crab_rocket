use colored::Colorize;

pub mod mkdir;
pub mod set_db_url;
pub mod time;
pub fn run_preload() {
    println!("{}", "Running preload...".blue());
    let _ = set_db_url::set_database_url_from_input().unwrap();
    mkdir::make_directory("upload");
    print!("{}\n", "Finished preload...".blue());
}

#[test]
fn test_run_preload() {
    run_preload()
}
