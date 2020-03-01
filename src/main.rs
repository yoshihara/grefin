use std::env;

fn find_files<T: Into<String>>(name: T, dir: T) {
    println!("{} in {}", name.into(), dir.into());
}
// fn find_file2<T>(name: T) where T: Into<String> {
//     println!("{}", name.into());
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        find_files(&args[1], &args[2]);
    } else {
        println!("not enough argument");
    }
}
