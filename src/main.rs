extern crate grefin;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grefin::Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("{:?}", config);
    if let Err(err) = grefin::run(config) {
        println!("{}", err);
        process::exit(1);
    };
}
