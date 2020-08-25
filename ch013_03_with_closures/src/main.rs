use std::env;
use std::process;
use ch012_minigrep::Config;


fn main() {
    //We'll change config to accept an iterator rather than a vector of args:

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);    //note the e, which prints to stderr
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = ch012_minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


