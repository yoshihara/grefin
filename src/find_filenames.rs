use super::*;

pub fn find_files(config: &Config) -> Result<(), String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    find_files_in_directory(&mut stdout, &config.query, &config.path)
}

fn find_files_in_directory<W: Write>(w: &mut W, query: &str, dir: &str) -> Result<(), String>{
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {}", dir, e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                let filename_str = filename.to_str().ok_or("converting path error".to_string())?;

                if filename_str == ".git" {
                    continue;
                } else if path.is_dir() {
                    if filename_str.to_ascii_lowercase().contains(query) {
                        writeln!(w, "dirname: {}", path.display()).expect("output error");
                    }
                    if let Err(err) = find_files_in_directory(w, query, path.to_str().ok_or("converting path error".to_string())?) {
                        return Err(err);
                    }
                } else {
                    if filename_str.to_ascii_lowercase().contains(query) {
                        writeln!(w, "filename: {}", path.display()).expect("output error");
                   }
                }
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod test_find_files_in_directory {
    use super::*;

    #[test]
    fn test_found_file() {
        let mut output_stub = Vec::<u8>::new();
        assert!(find_files_in_directory(&mut output_stub, "hoge", "fixtures/").is_ok());

        assert_eq!("filename: fixtures/test/foo/hoge.txt\n", String::from_utf8(output_stub).unwrap());
    }

    #[test]
    fn test_not_found() {
        let mut output_stub = Vec::<u8>::new();
        assert!(find_files_in_directory(&mut output_stub, "notfound", "fixtures/").is_ok());

        assert_eq!("", String::from_utf8(output_stub).unwrap());
    }

    #[test]
    fn test_found_upcase_file() {
        let mut output_stub = Vec::<u8>::new();
        assert!(find_files_in_directory(&mut output_stub, "fuga", "fixtures/").is_ok());

        assert_eq!("filename: fixtures/test/foo/FUGA.txt\n", String::from_utf8(output_stub).unwrap());
    }

    #[test]
    fn test_found_dir() {
        let mut output_stub = Vec::<u8>::new();
        assert!(find_files_in_directory(&mut output_stub, "bar", "fixtures/").is_ok());

        assert_eq!("dirname: fixtures/test/bar\nfilename: fixtures/test/bar/bar.txt\n", String::from_utf8(output_stub).unwrap());
    }

    #[test]
    fn test_not_existed_dir_as_path() {
        let mut output_stub = Vec::<u8>::new();
        assert!(find_files_in_directory(&mut output_stub, "bar", "notfound/").is_err());
    }
}