pub mod config;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::Read,
    process::Command,
};

// Opens the default editor with a temporary file
pub fn run(content: &mut String, title: &str) {
    let mut file_path = temp_dir();
    let file_name = title.to_lowercase().replace(" ", "-") + ".md";
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
        .read_to_string(content);
}
