use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}
