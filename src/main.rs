use sage::Config;

use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args());

    if let Err(e) = config.prepare_env() {
        eprintln!("Environment Error!: {}", e);
        process::exit(1);
    };

    sage::run(config).unwrap();
}
