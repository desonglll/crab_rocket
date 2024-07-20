use colored::Colorize;

pub mod mkdir;
pub mod time;
pub fn run_preload() {
    println!("{}", "Running preload...".blue());
    mkdir::make_directory("upload");
    print!("{}\n", "Finished preload...".blue());
}

#[test]
fn test_run_preload() {
    run_preload()
}
