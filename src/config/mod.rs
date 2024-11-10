mod command;

use crate::errors::SoupError;
use command::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ZolaConfig {
    pub dir: PathBuf,
    pub base_url: Option<String>,
    pub output_dir: Option<String>,
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
    const REMOTE_PATH: &'static str = "config.toml";

    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Config, SoupError> {
        args.next();

        // Parse command
        let cmd = match args.next() {
            Some(s) => Command::from(s),
            None => Ok(Command::Add),
        }?;

        // Parse local config
        let config_path = PathBuf::from(env::var("HOME").unwrap()).join(".config/soup.cfg");
        let local_config = match fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(_) => return Err(SoupError::MissingConfig),
        };
        let dir = Self::parse_local(local_config)?;

        // Parse remote config
        let remote_path = format!("{}/{}", dir, Config::REMOTE_PATH);
        let remote_config = match fs::read_to_string(remote_path) {
            Ok(c) => c,
            Err(_) => return Err(SoupError::MissingConfig),
        };
        let (base_url, output_dir) = Self::parse_remote(remote_config)?;

        Ok(Self {
            cmd,
            tags: vec![],
            title: String::new(),
            content: String::new(),
            zola: ZolaConfig {
                dir: PathBuf::from(&dir),
                base_url,
                output_dir,
            },
        })
    }

    // Parses contents of local config file
    fn parse_local(content: String) -> Result<String, SoupError> {
        for line in content.lines() {
            if line.starts_with("zola_dir") {
                return Ok(Self::trim_value(line.trim_start_matches("zola_dir")));
            }
        }

        return Err(SoupError::InvalidConfig);
    }

    // Parses contents of config.toml at Zola directory
    fn parse_remote(content: String) -> Result<(Option<String>, Option<String>), SoupError> {
        let mut base_url = None;
        let mut output_dir = None;

        for line in content.lines() {
            if line.starts_with("base_url") {
                base_url = Some(Self::trim_value(line.trim_start_matches("base_url")));
            }

            if line.starts_with("output_dir") {
                let out = Self::trim_value(line.trim_start_matches("output_dir"));
                output_dir = Some(out);
            }
        }

        return Ok((base_url, output_dir));
    }

    // Trims value from key-value pair
    fn trim_value(value: &str) -> String {
        let trimmed = value.trim().replace("=", "");
        let trimmed = trimmed.replace("\"", "");
        let mut bytes = trimmed.trim().as_bytes();

        if bytes[bytes.len() - 1] == b'/' {
            bytes = &bytes[..bytes.len() - 1];
        }

        std::str::from_utf8(bytes).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_local_multi_line() {
        let content = String::from("zola_dir=/path/to/zola/dir\noutput_dir=/path/to/output/dir");
        let result = Config::parse_local(content).unwrap();

        assert_eq!(Some(result), Some(String::from("/path/to/zola/dir")));
    }

    #[test]
    fn test_parse_local_trailing_slash() {
        let ml = String::from("zola_dir=/path/to/zola/dir/\noutput_dir=/path/to/output/dir\n\n\n");
        let sl = String::from("zola_dir=/path/to/zola/dir/");

        assert_eq!(
            Some(Config::parse_local(ml).unwrap()),
            Some(String::from("/path/to/zola/dir"))
        );

        assert_eq!(
            Some(Config::parse_local(sl).unwrap()),
            Some(String::from("/path/to/zola/dir"))
        );
    }

    #[test]
    fn test_parse_remote() {
        let (base_url1, output_dir1) = Config::parse_remote(String::from(
            "base_url = \"https://example.com\"\noutput_dir = \"/path/to/output/dir\"",
        ))
        .unwrap();
        let (base_url2, output_dir2) = Config::parse_remote(String::from(
            "base_url=\"https://example.com\"\noutput_dir=\"/path/to/output/dir\"",
        ))
        .unwrap();
        let (base_url3, output_dir3) = Config::parse_remote(String::from(
            "base_url= \"https://example.com\"\noutput_dir= \"/path/to/output/dir\"",
        ))
        .unwrap();
        let (base_url4, output_dir4) = Config::parse_remote(String::from(
            "base_url =\"https://example.com\"\noutput_dir =\"/path/to/output/dir\"",
        ))
        .unwrap();

        assert_eq!(base_url1, Some(String::from("https://example.com")));
        assert_eq!(output_dir1, Some(String::from("/path/to/output/dir")));
        assert_eq!(base_url2, Some(String::from("https://example.com")));
        assert_eq!(output_dir2, Some(String::from("/path/to/output/dir")));
        assert_eq!(base_url3, Some(String::from("https://example.com")));
        assert_eq!(output_dir3, Some(String::from("/path/to/output/dir")));
        assert_eq!(base_url4, Some(String::from("https://example.com")));
        assert_eq!(output_dir4, Some(String::from("/path/to/output/dir")));
    }

    #[test]
    fn test_trim_value() {
        let value1 = " = \"https://example.com\"";
        let value2 = " = \"/path/to/output/dir\"";
        let value3 = "https://example.com\"";
        let value4 = "\"/path/to/output/dir\"";
        let value5 = "=\"/path/to/output/dir/\"";
        let value6 = " =\"/path/to/output/dir\"";

        assert_eq!(
            Config::trim_value(value1),
            String::from("https://example.com")
        );
        assert_eq!(
            Config::trim_value(value2),
            String::from("/path/to/output/dir")
        );
        assert_eq!(
            Config::trim_value(value3),
            String::from("https://example.com")
        );
        assert_eq!(
            Config::trim_value(value4),
            String::from("/path/to/output/dir")
        );
        assert_eq!(
            Config::trim_value(value5),
            String::from("/path/to/output/dir")
        );
        assert_eq!(
            Config::trim_value(value6),
            String::from("/path/to/output/dir")
        );
    }
}
