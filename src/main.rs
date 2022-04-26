use std::env;
use std::process;
// use std::error::Error;
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Something went wrong when parsing the file. Err: {}",err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Error Occured. {} \nExiting...",e)
    }
    
    

}
