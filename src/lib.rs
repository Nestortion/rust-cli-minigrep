use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query_string: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(query_string: &str, file_path: &str, case_sensitive: bool) -> Config {
        Config {
            query_string: query_string.to_string(),
            file_path: file_path.to_string(),
            case_insensitive: case_sensitive,
        }
    }
}

pub fn parse_config<'a>(mut args: impl Iterator<Item = String>) -> Result<Config, &'a str> {
    args.next();
    let query_string = match args.next() {
        Some(query_string) => query_string,
        None => return Err("Did not receive query string"),
    };
    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => return Err("Did not receive file path"),
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();

    Ok(Config::new(
        query_string.as_str(),
        file_path.as_str(),
        case_sensitive,
    ))
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_to_string(config.file_path)?;

    for line in {
        if config.case_insensitive {
            search_insensitive(contents.as_str(), config.query_string.as_str())
        } else {
            search(contents.as_str(), config.query_string.as_str())
        }
    } {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(contents: &'a str, word_to_search: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(word_to_search))
        .collect()
}

pub fn search_insensitive<'a>(contents: &'a str, word_to_search: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            line.to_lowercase()
                .contains(word_to_search.to_lowercase().as_str())
        })
        .collect()
}
