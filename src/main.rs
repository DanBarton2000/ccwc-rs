use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='c', action)]
    bytes: bool,
    #[clap(short='l', action)]
    lines: bool,
    #[clap(short='w', action)]
    words: bool,
    #[clap(short='m', action)]
    characters: bool,
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut result = String::new();

    let all: bool = !args.bytes && !args.lines && !args.words && !args.characters;

    if args.bytes || all {
        result.push_str(&get_bytes(&args.path).unwrap().to_string());
        result.push_str(" ");
    }

    if args.lines || all {
        result.push_str(&get_lines(&args.path).unwrap().to_string());
        result.push_str(" ");
    }

    if args.words || all {
        result.push_str(&get_words(&args.path).unwrap().to_string());
        result.push_str(" ");
    }

    if args.characters || all {
        result.push_str(&get_characters(&args.path).unwrap().to_string());
        result.push_str(" ");
    }

    result.push_str(&args.path.to_str().unwrap());

    println!("{}", result);
}

fn get_bytes(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    Ok(fs::metadata(&path_buf)?.len())
}

fn get_lines(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    let file = File::open(&path_buf)?;
    let reader = BufReader::new(file);

    let mut count: u64 = 0;
    for _ in reader.lines() {
        count += 1;
    }
    Ok(count)
}

fn get_words(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    let file = File::open(&path_buf)?;
    let reader = BufReader::new(file);

    let mut words: u64 = 0;
    for line in reader.lines() {
        words += line?.split_whitespace().count() as u64;
    }

    Ok(words)
}

fn get_characters(path_buf: &PathBuf) -> Result<u64, std::io::Error> {
    let file = File::open(&path_buf)?;
    let mut reader = BufReader::new(file);
    let mut string = String::new();

    let mut characters: u64 = 0;
    while reader.read_line(&mut string)? > 0 {
        characters += string.chars().count() as u64;
        string.clear();
    }

    Ok(characters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bytes() {
        assert_eq!(get_bytes(&PathBuf::from(".\\test\\test.txt")).unwrap(), 342190);
    }

    #[test]
    fn test_get_bytes_error() {
        assert!(get_bytes(&PathBuf::from(".\\test\\not_a_file.txt")).is_err());
    }

    #[test]
    fn test_get_lines() {
        assert_eq!(get_lines(&PathBuf::from(".\\test\\test.txt")).unwrap(), 7145);
    }

    #[test]
    fn test_get_words() {
        assert_eq!(get_words(&PathBuf::from(".\\test\\test.txt")).unwrap(), 58164);
    }

    #[test]
    fn test_get_characters() {
        assert_eq!(get_characters(&PathBuf::from(".\\test\\test.txt")).unwrap(), 339292);
    }
}