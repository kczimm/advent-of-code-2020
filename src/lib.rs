use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn load_file(filename: &str) -> Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|s| s.to_owned()).collect())
}
