use std::env;
use std::process;
use ch012_minigrep::Config;


//guidelines for splitting of concerns:
//  --Split your program into a main.rs and a lib.rs and move program logic to lib.rs
//  --As long as your program argument parsing is small, it can stay in main.rs
//  --If the argument parsing grows complex, split it out to lib.rs



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
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


