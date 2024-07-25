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

pub fn parse_config(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
        Err("Not enough arguments")
    } else {
        let query_string = &args[1];
        let file_path = &args[2];

        let case_sensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config::new(query_string, file_path, case_sensitive))
    }
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
    let mut lines_with_word = Vec::new();

    for line in contents.lines() {
        if line.contains(word_to_search) {
            lines_with_word.push(line);
        }
    }

    lines_with_word
}

pub fn search_insensitive<'a>(contents: &'a str, word_to_search: &str) -> Vec<&'a str> {
    let mut lines_with_word = Vec::new();

    for line in contents.lines() {
        if line
            .to_lowercase()
            .contains(word_to_search.to_lowercase().as_str())
        {
            lines_with_word.push(line);
        }
    }

    lines_with_word
}
