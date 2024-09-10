use std::env;
use std::process;

use notes::config::Config;

fn main() {
    let mut config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    notes::get_title(&mut config);
    notes::get_content(&mut config);
    notes::get_tags(&mut config);
    notes::write(&config);
    notes::build(&config);
    notes::publish(&config);
}
