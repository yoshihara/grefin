pub fn find_files<T: Into<String>>(name: T, dir: T) {
    println!("{} in {}", name.into(), dir.into());
}
// fn find_file2<T>(name: T) where T: Into<String> {
//     println!("{}", name.into());
// }
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