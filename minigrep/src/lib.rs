use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguemnts")
        }
        Ok(Config::new(args))
    }

    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }   
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}
