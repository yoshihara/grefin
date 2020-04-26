use std::fs::File;
use std::io::Read;
use super::*;

pub fn grep_files(config: &Config) -> Result<(), String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    grep_files_in_directory(&mut stdout, &config.query, &config.path)
}

fn grep_files_in_directory<W: Write>(w: &mut W, query: &str, dir: &str) -> Result<(), String>{
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {}", dir, e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_str().ok_or("converting path error".to_string())?;

                if filename_str == ".git" {
                    continue;
                } else if path.is_dir() {
                    if let Err(err) = grep_files_in_directory(w, query, path.to_str().ok_or("converting path error".to_string())?) {
                        return Err(err);
                    }
                } else {
                    let mut f = File::open(path.as_os_str()).expect("file not found");
                    let mut contents = String::new();
                    let mut count = 0;
                    f.read_to_string(&mut contents)
                        .expect("something went wrong reading the file");
                    for line in contents.lines() {
                        count += 1;
                        if line.contains(query) {
                            writeln!(w, "{}:{}: {}", path.to_str().unwrap(), count, line).expect("output error");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test_grep_files_in_directory {
    use super::*;

    #[test]
    fn test_hit_file() {
        let mut output_stub = Vec::<u8>::new();
        assert!(grep_files_in_directory(&mut output_stub, "This is test", "fixtures/").is_ok());

        assert_eq!("fixtures/test/bar/bar.txt:1: This is test text for grep files.\n", String::from_utf8(output_stub).unwrap());
    }

    #[test]
    fn test_hit_no_file() {
        let mut output_stub = Vec::<u8>::new();
        assert!(grep_files_in_directory(&mut output_stub, "no hit", "fixtures/").is_ok());

        assert_eq!("", String::from_utf8(output_stub).unwrap());
    }
