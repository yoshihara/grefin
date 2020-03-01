mod find_filenames;
mod grep_files;

use crate::find_filenames::find_files;
use crate::grep_files::grep_files;

use std::fs;
use std::io::{self, Write};

pub fn run(config: Config) -> Result<(), String> {
    find_files(&config)?;
    grep_files(&config)
}

#[derive(Debug,PartialEq)]
pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() > 2 {
            Ok(Config { query: (&args[1]).to_ascii_lowercase().to_string(), path: (&args[2]).to_string() })
        } else {
            Err("not enough argument")
        }
    }
}

#[cfg(test)]
mod test_config {
    use super::*;

    #[test]
    fn test_new_with_enough_arguments() {
        let args = vec!("/path/to/executable".to_string(), "query".to_string(), "path".to_string());

        assert_eq!(Ok(Config { query: "query".to_string(), path: "path".to_string()}), Config::new(&args));
    }

    #[test]
    fn test_new_without_enough_arguments() {
        let args = vec!("query".to_string());

        assert!(Config::new(&args).is_err());
    }
}