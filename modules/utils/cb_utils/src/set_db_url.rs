use std::env;
use std::io::{self, Write};

use colored::Colorize;

pub fn set_database_url_from_input() -> io::Result<()> {
    // Default DATABASE_URL
    const DEFAULT_DATABASE_URL: &str = "postgres://postgres:070011@localhost:5432/hello_rocket";

    // Prompt the user to input DATABASE_URL
    print!(
        "{}{}\n",
        "Please enter DATABASE_URL (or press Enter to use default):\nDefault: ".blue(),
        DEFAULT_DATABASE_URL.green()
    );
    io::stdout().flush()?;

    // Read user input
    let mut database_url = String::new();
    io::stdin().read_line(&mut database_url)?;

    // Remove trailing newline character
    let database_url = database_url.trim();

    // Use default if input is empty
    let database_url = if database_url.is_empty() {
        DEFAULT_DATABASE_URL
    } else {
        database_url
    };

    // Set the environment variable
    env::set_var("DATABASE_URL", database_url);

    // Output the set environment variable to confirm
    let db_url =
        env::var("DATABASE_URL").expect("Failed to retrieve DATABASE_URL environment variable");
    println!("The set DATABASE_URL is: {}", db_url);

    Ok(())
}
