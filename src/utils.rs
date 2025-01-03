use anyhow::Result;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File docs not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File docs not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File docs not exist"));
    }

    #[test]
    fn test_verify_path() {
        assert_eq!(verify_path("."), Ok(".".into()));
        assert_eq!(verify_path("fixtures"), Ok("fixtures".into()));
        assert_eq!(
            verify_path("/tests"),
            Err("Path does not exist or is not a directory")
        );
    }
}
