use std::env;
use std::process;

use notes_rs::config::Config;
use notes_rs::errors;

fn main() {
    let mut config = Config::new(env::args()).unwrap_or_else(|err| {
        errors::describe_error(err);
        process::exit(1);
    });

    notes_rs::get_title(&mut config);
    notes_rs::get_content(&mut config);
    notes_rs::get_tags(&mut config);
    notes_rs::write(&config);
    notes_rs::build(&config);
    notes_rs::publish(&config);
}
