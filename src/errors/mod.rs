use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum SoupError {
    MissingConfig,
    InvalidConfig,
    InvalidCommand,
}

/// Prints out error description based on type
pub fn describe_error(err: SoupError) {
    match err {
        SoupError::MissingConfig => missing_config(),
        SoupError::InvalidCommand => invalid_command(),
        SoupError::InvalidConfig => invalid_config(),
    }
}

fn missing_config() {
    eprintln!("[Error]: Missing config file.");
    println!("-------");
    println!("Attempting to create one...");

    let config_path = PathBuf::from(env::var("HOME").unwrap()).join(".config/soup.cfg");
    let contents = "zola_dir=/path/to/zola/dir";

    match fs::write(&config_path, contents) {
        Ok(_) => println!("Created: {}", config_path.display()),
        Err(e) => eprintln!("Error creating config file: {}", e),
    }
}

fn invalid_config() {
    eprintln!("[Error]: Invalid config file.");
    println!("-------");
    println!("Zola directory isn't set or doesn't exist.");
}

fn invalid_command() {
    eprintln!("[Error]: Invalid command");
    println!("-------");
    println!("Available commands:");
    println!("  add      - Adds a new note");
    println!("");
}
