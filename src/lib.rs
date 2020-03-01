use std::fs;
use std::io::{self, Write};

pub fn run(config: Config) {
    find_files(config);
}

fn find_files(config: Config) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    find_files_in_directory(&mut stdout, &config.query, &config.path);
}

fn find_files_in_directory<W: Write>(w: &mut W, query: &str, dir: &str) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                if path_str.ends_with(".git") {
                    continue;
                } else if path.is_dir() {
                    find_files_in_directory(w, query, path_str)
                } else {
                    if path_str.to_ascii_lowercase().contains(query) {
                        writeln!(w, "filename: {}", path_str).expect("output error");
                   }
                }
            }
        }
    }
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

#[test]
fn test_find_files_in_directory_with_found() {
    let mut output_stub = Vec::<u8>::new();
    find_files_in_directory(&mut output_stub, "hoge", "fixtures/");

    assert_eq!(String::from_utf8(output_stub).unwrap(), "filename: fixtures/test/foo/hoge.txt\n");
}

#[test]
fn test_find_files_in_directory_without_found() {
    let mut output_stub = Vec::<u8>::new();
    find_files_in_directory(&mut output_stub, "notfound", "fixtures/");

    assert_eq!(String::from_utf8(output_stub).unwrap(), "");
}

#[test]
fn test_find_files_in_directory_with_found_upcase() {
    let mut output_stub = Vec::<u8>::new();
    find_files_in_directory(&mut output_stub, "fuga", "fixtures/");

    assert_eq!(String::from_utf8(output_stub).unwrap(), "filename: fixtures/test/foo/FUGA.txt\n");
}


#[test]
fn test_find_files_in_directory_with_found_dir() {
    let mut output_stub = Vec::<u8>::new();
    find_files_in_directory(&mut output_stub, "foo", "fixtures/");

    assert_eq!(String::from_utf8(output_stub).unwrap(), "filename: fixtures/test/foo/FUGA.txt\nfilename: fixtures/test/foo/hoge.txt\n");
}
