use std::env;
use std::process;

use rgrep;
use rgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: {}", err);
        process::exit(1);
    });

    if let Err(e) = rgrep::run(config) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }
}