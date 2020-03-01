pub fn run(config: Config) {
    find_files(config);
}

fn find_files(config: Config) {
    println!("{} in {}", config.query, config.path);
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() > 2 {
            Ok(Config { query: (&args[1]).to_string(), path: (&args[2]).to_string() })
        } else {
            Err("not enough argument")
        }
    }
}