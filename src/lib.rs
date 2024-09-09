pub mod config;

use std::io::prelude::*;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::Read,
    process::Command,
};

use chrono::Local;
use config::Config;

// Get title from user input, and update config
pub fn get_title(config: &mut Config) {
    println!("\nEnter title: ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read title");

    if input.trim().is_empty() {
        eprintln!("Error: Title cannot be empty");
        panic!("Empty title");
    }

    config.title = input.trim().to_string();
}

// Get tags from user input, and update config
pub fn get_tags(config: &mut Config) {
    println!("Enter tags (comma separated): ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read tags");

    if !input.trim().is_empty() {
        config.tags = input.split(',').map(|tag| tag.trim().to_string()).collect();
    }
}

// Opens the default editor with a temporary file
pub fn get_content(config: &mut Config) {
    let mut file_path = temp_dir();
    let mut content = String::new();
    let file_name = config.title.to_lowercase().replace(" ", "-") + ".md";
    let editor = var("EDITOR").unwrap();

    file_path.push(file_name);
    File::create(&file_path).expect("Could not create temp file");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Failed opening editor");

    // TODO: Error handling
    let _result = File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut content);

    config.content = content;
}

pub fn write(config: &mut Config) {
    let file_name = config.title.to_lowercase().replace(" ", "-") + ".md";
    let file_path = format!("{}/{}", config.content_dir, file_name);

    let mut file = File::create(&file_path).expect("Could not create file at content_dir");

    // append date & title to content
    let mut prefix = String::from("+++\n");
    prefix.push_str(&format!("title = \"{}\"\n", config.title));
    prefix.push_str(&format!("date = {}\n", Local::now().format("%Y-%m-%d")));

    if config.tags.len() > 0 {
        prefix.push_str(&format!("\n[taxonomies]\ntags = {:?}\n", config.tags));
    }

    prefix.push_str("+++\n\n");

    config.content.insert_str(0, &prefix);

    match file.write_all(config.content.as_bytes()) {
        Ok(_) => println!("\nSuccessfully wrote to file: {}", file_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
