use std::fs;
use std::io::{self, Write};

pub fn run(config: Config) -> Result<(), String>{
    find_files(config)
}

fn find_files(config: Config) -> Result<(), String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    find_files_in_directory(&mut stdout, &config.query, &config.path)
}

fn find_files_in_directory<W: Write>(w: &mut W, query: &str, dir: &str) -> Result<(), String>{
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let path_str = path.to_str().unwrap();

            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_str().unwrap();
                if filename_str == ".git" {
                    continue;
                } else if path.is_dir() {
                    if filename_str.to_ascii_lowercase().contains(query) {
                        writeln!(w, "dirname: {}", path_str).expect("output error");
                    }
                    if let Err(err) = find_files_in_directory(w, query, path_str) {
                        return Err(err);
                    }
                } else {
                    if filename_str.to_ascii_lowercase().contains(query) {
                        writeln!(w, "filename: {}", path_str).expect("output error");
                   }
                }
            }
        }
    }
    Ok(())
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
    assert!(find_files_in_directory(&mut output_stub, "hoge", "fixtures/").is_ok());

    assert_eq!(String::from_utf8(output_stub).unwrap(), "filename: fixtures/test/foo/hoge.txt\n");
}

#[test]
fn test_find_files_in_directory_without_found() {
    let mut output_stub = Vec::<u8>::new();
    assert!(find_files_in_directory(&mut output_stub, "notfound", "fixtures/").is_ok());

    assert_eq!(String::from_utf8(output_stub).unwrap(), "");
}

#[test]
fn test_find_files_in_directory_with_found_upcase() {
    let mut output_stub = Vec::<u8>::new();
    assert!(find_files_in_directory(&mut output_stub, "fuga", "fixtures/").is_ok());

    assert_eq!(String::from_utf8(output_stub).unwrap(), "filename: fixtures/test/foo/FUGA.txt\n");
}


#[test]
fn test_find_files_in_directory_with_found_dir() {
    let mut output_stub = Vec::<u8>::new();
    assert!(find_files_in_directory(&mut output_stub, "bar", "fixtures/").is_ok());

    assert_eq!("dirname: fixtures/test/bar\nfilename: fixtures/test/bar/bar.txt\n", String::from_utf8(output_stub).unwrap());
}
