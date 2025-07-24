use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path })

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        //
        // Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("should have been able to read the file");
    eprintln!("with text:\n{contents}");

    Ok(())
}
