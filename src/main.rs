use std::env;
use std::process;

use baby_sensor::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = baby_sensor::run(config) {
        // --snip--
        println!("Application error: {}", e);

        process::exit(1);
    }
}