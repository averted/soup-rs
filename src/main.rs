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
    notes::write(&mut config);

    println!("Config: {:?}", config);

    // CD into CONTENT_DIR & Run zola build

    // Print stdout of zola build

    // Run git -A with commit message of "New post: file_name"

    // Run git commit

    // Run git push
}
