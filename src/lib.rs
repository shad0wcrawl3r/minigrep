use std::error::Error;
use std::fs;
use std::env;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"],insensitive_search(query,contents));
    }

}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
            results.push(line)
        }
    }
    results
     
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line)
        }
    }
    results
     
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let str_from_file = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query,&str_from_file)
    } else {
        insensitive_search(&config.query,&str_from_file)
    };

    
    for line in results{
        println!("{}",line)
    }
    Ok(())
}