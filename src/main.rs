use std::env;
use std::process;

use notes::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut content = String::new();

    notes::run(&mut content, &config.title);

    println!("Config: {:?}", config);
    println!("Content: {}", content);
}
