//! [![url-github]](https://github.com/averted/soup-rs)
//! [![url-crates]](https://crates.io/crates/soup-rs)
//!
//! [url-github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [url-crates]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//!
//! `soup-rs` is a cli tool to help you manage your [Zola](https://github.com/getzola/zola) site.
//!
//! ## Usage:
//! ```sh
//! $ soup-rs <COMMAND = "add">
//!
//! Arguments:
//!   <COMMAND>             Command to execute (i.e. "add")
//! ```

pub mod config;
pub mod errors;

use std::io::Write;
use std::path::PathBuf;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::Read,
    process::Command,
};

use chrono::Local;
use config::Config;

/// Gets title from user input, and updates config
pub fn get_title(config: &mut Config) {
    println!("\nEnter title: ");
    println!("------------");

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

/// Gets tags from user input, and updates config
pub fn get_tags(config: &mut Config) {
    if let Some(output_dir) = &config.zola.output_dir {
        let mut out_path = PathBuf::from(output_dir);

        if out_path.is_relative() {
            out_path = config.zola.dir.join(out_path);
        }

        let output = Command::new("ls")
            .arg(out_path.join("tags"))
            .output()
            .expect("Failed to list tags");

        // Parse available tags from output directory
        let tags = String::from_utf8(output.stdout).unwrap();
        let tags = parse_invalid_tags(tags.split('\n').collect());

        if tags.len() > 0 {
            println!("Enter tags [{}]:", tags.join(", "));
            println!("----------");
        } else {
            println!("Enter tags (comma separated): ");
            println!("----------");
        }
    }

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read tags");

    if !input.trim().is_empty() {
        config.tags = input.split(',').map(|tag| tag.trim().to_string()).collect();
    }
}

/// Opens default editor with a temporary file
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

    let _result = File::open(file_path)
        .expect("Could not open file")
        .read_to_string(&mut content);

    config.content = content;
}

/// Writes note to Zola content directory
pub fn write(config: &Config) {
    let file_name = config.title.to_lowercase().replace(" ", "-") + ".md";
    let file_path = config.zola.dir.join("content").join(file_name);

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
        Ok(_) => println!(
            "\n> Successfully wrote to file:\n> {}\n",
            file_path.display()
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Builds the site using Zola
pub fn build(config: &Config) {
    let mut zola = Command::new("zola");

    zola.arg("build")
        .current_dir(&config.zola.dir)
        .status()
        .expect("Failed to build site");
}

/// Publishes changes to git
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

// Removes invalid tags
fn parse_invalid_tags(tags: Vec<&str>) -> Vec<String> {
    const INVALID_TAGS: [&str; 5] = ["", ".", "..", "index.md", "index.html"];

    tags.iter()
        .filter(|tag| !INVALID_TAGS.contains(tag))
        .map(|tag| tag.to_string())
        .collect()
}
