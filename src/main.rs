use std::{
    fs::File,
    io::{Error, Read},
};

fn read_content_from_file(path: &str) -> Result<String, Error> {
    let mut content: String = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let path = "README.md";
    let content = match read_content_from_file(path) {
        Ok(content) => content,
        Err(error) => panic!("Error: {}, Path: {}", error, path),
    };
    println!("path: {}, content: {}", path, content);
}
