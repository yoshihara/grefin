use std::fs;

pub fn run(config: Config) {
    find_files(config);
}

fn find_files(config: Config) {
    find_files_in_directory(&config.query, &config.path);
}

fn find_files_in_directory(query: &str, dir: &str) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                if path_str.ends_with(".git") {
                    continue;
                } else if path.is_dir() {
                    find_files_in_directory(query, path_str)
                } else {
                    if path_str.to_ascii_lowercase().contains(query) {
                        println!("found {}!", path_str);
                   }
                }
            }
        }
    }
}
#[derive(Debug)]
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