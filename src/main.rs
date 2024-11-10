use std::env;
use std::process;

use soup_rs::config::Config;
use soup_rs::errors;

fn main() {
    let mut config = Config::new(env::args()).unwrap_or_else(|err| {
        errors::describe_error(err);
        process::exit(1);
    });

    soup_rs::get_title(&mut config);
    soup_rs::get_content(&mut config);
    soup_rs::get_tags(&mut config);
    soup_rs::write(&config);
    soup_rs::build(&config);
    soup_rs::publish(&config);
}
