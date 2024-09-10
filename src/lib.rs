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

    println!("");

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

// Builds the site using Zola
pub fn build(config: &Config) {
    let mut zola = Command::new("zola");

    zola.arg("build")
        .current_dir(&config.zola.dir)
        .status()
        .expect("Failed to build site");
}

// Publishes changes to git
pub fn publish(config: &Config) {
    let mut git_add = Command::new("git");
    let mut git_commit = Command::new("git");
    let mut git_push = Command::new("git");

    git_add
        .arg("add")
        .arg("-A")
        .current_dir(&config.zola.dir)
        .status()
        .expect("Failed to add files");

    git_commit
        .arg("commit")
        .arg("-m")
        .arg(&format!("New post: \"{}\"", config.title))
        .current_dir(&config.zola.dir)
        .status()
        .expect("Failed to commit changes");

    match &config.zola.base_url {
        Some(url) => {
            println!("\n> Changes to be published at: {}", url);
        }
        None => (),
    }

    git_push
        .arg("push")
        .current_dir(&config.zola.dir)
        .status()
        .expect("Failed to publish changes");
}

// Writes the content to file at zola.dir
pub fn write(config: &Config) {
    let file_name = config.title.to_lowercase().replace(" ", "-") + ".md";
    let file_path = format!("{}/content/{}", config.zola.dir, file_name);

    let mut file =
        File::create(&file_path).expect("Could not create file at zola content directory");

    // append date & title prefix to content
    let mut result = String::new();
    let mut prefix = String::new();
    prefix.push_str("+++\n");
    prefix.push_str(&format!("title = \"{}\"\n", config.title));
    prefix.push_str(&format!("date = {}\n", Local::now().format("%Y-%m-%d")));

    if config.tags.len() > 0 {
        prefix.push_str(&format!("\n[taxonomies]\ntags = {:?}\n", config.tags));
    }

    prefix.push_str("+++\n\n");

    result.insert_str(0, &prefix);
    result.insert_str(prefix.len(), &config.content);

    match file.write_all(result.as_bytes()) {
        Ok(_) => println!("\n> Successfully wrote to file:\n> {}\n", file_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
