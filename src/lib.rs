use std::{env, error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.clone())?;

    // println!("With text:\n{contents}");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[derive(Debug)]
pub struct Config {
    pub case_sensitive: bool,
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => Err("Didn't get query string")?,
        };

        let filename = match args.next() {
            Some(q) => q,
            None => Err("Didn't get filename")?,
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        );
    }
}
