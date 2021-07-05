use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn load_file_by_lines(filename: &str) -> Result<Vec<String>> {
    let contents = load_file(filename)?;
    Ok(contents.lines().map(|s| s.to_owned()).collect())
}

pub fn load_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
