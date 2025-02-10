use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Defining a build func because new funcs are not 
    // expected to have the possibility of erroring
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone() is not the most efficient method to use here,
        // it's used for simplicity in the lesson
        let query = args[1].clone();
        let file_path = args[2].clone();

        // If IGNORE_CASE is undefined it will return an error, 
        // and is_ok() will return false
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// A lifetime is required so contents is used
// since the return value will be a slice of it
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// A lifetime is required so contents is used
// since the return value will be a slice of it
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

// Box<dyn Error> is a trait object, more info below:
// Box<dyn Error> means the func will return a type that implements the Error trait,
// but I don't have to specify what specific type the return val will be.
// This gives me flexibility to return dif types in dif error cases.
// The dyn keyword is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will cause the error handling responsibility
    // to be passed to the function calling run()
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // The backslash after the opening double quote tells Rust not to put
        // a newline character at the beg of the contents of the string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // The backslash after the opening double quote tells Rust not to put
        // a newline character at the beg of the contents of the string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
