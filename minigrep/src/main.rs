//use std::env;   //se usa para ver las variables
//use std::fs;    //para manipular archivos
use minigrep::Config;
use std::{env, process};

fn main() {
    //accede a las variables
    // let args: Vec<String> = env::args().collect();

    //let query = &args[1];
    //let file_path = &args[2];

    let config = Config::build(env::args()).unwrap_or_else(|err|{
        println!("problem parsing aregument: {err}");
        process::exit(1);
    }) /* parse_config(&args) */;

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//
//     Config { query, file_path }
// }
