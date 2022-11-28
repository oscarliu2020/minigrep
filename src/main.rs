use std::process;
use std::{env};

use minigrep::Config;
fn main() {
    let args:Vec<String>=env::args().collect();
    let config =Config::build(&args).unwrap();
    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);
    if let Err(e)=minigrep::run(config){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
