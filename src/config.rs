use std::fmt::Debug;
use std::fs;
use std::io::stdin;

fn parse_content_dir_from_config() -> Option<String> {
    let content = fs::read_to_string("config");

    let content = match content {
        Ok(c) => c,
        Err(_) => return None,
    };

    let dir = content
        .trim_start_matches("content_dir=")
        .trim_matches('"')
        .trim();

    // if last char is a slash, remove it
    let dir = if dir.ends_with('/') {
        &dir[..dir.len() - 1]
    } else {
        dir
    };

    if dir.is_empty() {
        None
    } else {
        Some(dir.to_string())
    }
}

pub enum Command {
    Add,
}

impl Command {
    pub fn from(s: String) -> Result<Command, &'static str> {
        match s.as_str() {
            "add" => Ok(Command::Add),
            _ => Err("Invalid command"),
        }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Add => write!(f, "Add"),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub cmd: Command,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub content_dir: String,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let cmd = match args.next() {
            Some(s) => Command::from(s),
            None => return Err("Missing command"),
        };

        if let Err(e) = &cmd {
            return Err(e);
        }

        let content_dir = match parse_content_dir_from_config() {
            Some(dir) => dir,
            None => {
                let mut buffer = String::new();

                println!("Missing config file.\nEnter content directory path:");

                match stdin().read_line(&mut buffer) {
                    Ok(size) => {
                        if size > 0 {
                            fs::write("config", format!("content_dir={}", buffer)).unwrap();
                        }
                    }
                    Err(_) => return Err("Could not read from stdin"),
                }
                buffer.trim().to_string()
            }
        };

        Ok(Self {
            cmd: cmd.unwrap(),
            tags: vec![],
            title: String::new(),
            content: String::new(),
            content_dir,
        })
    }
}
