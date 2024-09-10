use std::fmt::Debug;
use std::fs;
use std::io::stdin;

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
pub struct ZolaConfig {
    pub dir: String,
    pub base_url: Option<String>,
}

impl ZolaConfig {
    pub fn new() -> Self {
        Self {
            dir: String::new(),
            base_url: None,
        }
    }

    pub fn set_dir(&mut self, dir: String) {
        self.dir = dir;
    }

    pub fn set_base_url(&mut self, base_url: Option<String>) {
        self.base_url = base_url;
    }
}

#[derive(Debug)]
pub struct Config {
    pub cmd: Command,
    pub tags: Vec<String>,
    pub title: String,
    pub content: String,
    pub zola: ZolaConfig,
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

        let zola_dir = match parse_zola_dir_from_config() {
            Some(dir) => dir,
            None => {
                let mut buffer = String::new();

                println!("Missing config file.\nEnter content directory path:");

                match stdin().read_line(&mut buffer) {
                    Ok(size) => {
                        if size > 0 {
                            fs::write("config", format!("zola_dir={}", buffer)).unwrap();
                        }
                    }
                    Err(_) => return Err("Could not read from stdin"),
                }
                buffer.trim().to_string()
            }
        };

        let mut zola = ZolaConfig::new();
        zola.set_dir(zola_dir);

        if !zola.dir.is_empty() {
            zola.set_base_url(parse_zola_base_url_from_config(zola.dir.clone()));
        }

        Ok(Self {
            cmd: cmd.unwrap(),
            tags: vec![],
            title: String::new(),
            content: String::new(),
            zola,
        })
    }
}

fn parse_zola_dir_from_config() -> Option<String> {
    let content = fs::read_to_string("config");

    let content = match content {
        Ok(c) => c,
        Err(_) => return None,
    };

    let dir = content
        .trim_start_matches("zola_dir=")
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

fn parse_zola_base_url_from_config(root_dir: String) -> Option<String> {
    let path = format!("{}/config.toml", root_dir);
    let config = fs::read_to_string(path);

    let config = match config {
        Ok(c) => c,
        Err(_) => return None,
    };

    // find line that starts with base_url and extract the value
    let base_url_line = config
        .lines()
        .find(|line| line.starts_with("base_url"))
        .unwrap_or("");

    println!("base_url_line: {}", base_url_line);

    let base_url = base_url_line
        .trim_start_matches("base_url = ")
        .trim_matches('"')
        .trim();

    println!("base_url: {}", base_url);

    if base_url.is_empty() {
        None
    } else {
        Some(base_url.to_string())
    }
}
