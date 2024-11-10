use std::fs;

#[derive(Debug, PartialEq)]
pub enum NotesError {
    MissingConfig,
    InvalidConfig,
    InvalidCommand,
}

/// Prints out error description based on type
pub fn describe_error(err: NotesError) {
    match err {
        NotesError::MissingConfig => missing_config(),
        NotesError::InvalidCommand => invalid_command(),
        NotesError::InvalidConfig => invalid_config(),
    }
}

fn missing_config() {
    eprintln!("[Error]: Missing config file.");
    println!("-------");
    println!("Attempting to create one...");

    let name = "config.cfg";
    let contents = "zola_dir=/path/to/zola/dir";

    match fs::write(name, contents) {
        Ok(_) => println!("Created: {}", name),
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
