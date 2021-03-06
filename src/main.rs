use sage::Config;

use std::env;
use std::process;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration Error!: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = config.prepare_env() {
        eprintln!("Environment Error!: {}", e);
        process::exit(1);
    };

    if let Err(e) = sage::run(config) {
        eprintln!("Application Error!: {}", e);
    };
}
